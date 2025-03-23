import fs from "node:fs/promises";
import path from "node:path";
import process from "node:process";

import log from "~/util/log";
import exec from "~/util/exec";
import * as nix from "~/util/nix";

export const exists = async (path: string) => {
	try {
		await fs.access(path);
		return true;
	} catch (error) {
		return false;
	}
};

export enum Source {
	Path = "path",
	Git = "git",
	Github = "github",
	Gitlab = "gitlab",
	Sourcehut = "sourcehut",
	Tarball = "tarball",
}

export const getProjectRoot = async (base: string) => {
	let absolute = path.isAbsolute(base) ? base : path.resolve(base);
	let root = absolute;

	while (true) {
		if (root === "/") {
			log.fatal(`No Nilla project found at or above "${absolute}".`);
			process.exit(1);
		}

		try {
			await exists(path.join(root, "nilla.nix"));
			break;
		} catch (error) {
			root = path.dirname(root);
		}
	}

	return root;
};

export const isUrl = (uri: string) => {
	try {
		new URL(uri);
		return true;
	} catch (error) {
		return false;
	}
};

export const getRawUrl = (uri: string): URL => {
	try {
		return new URL(uri);
	} catch (error) {
		log.fatal(`Invalid URI: ${uri}`);
		process.exit(1);
	}
};

export const resolve = async (uri: string) => {
	if (uri.startsWith(".") || uri.startsWith("/")) {
		const root = await getProjectRoot(uri);

		return {
			source: Source.Path,
			path: root,
		};
	}

	if (uri.startsWith("path:")) {
		const root = uri.slice("path:".length);

		if (!(await exists(root))) {
			log.fatal(`Path not found: ${root}`);
			process.exit(1);
		}

		return {
			source: Source.Path,
			path: root,
		};
	}

	const url = getRawUrl(uri);

	switch (url.protocol) {
		case "git:": {
			const info = {
				url: url.pathname,
				rev: url.searchParams.get("rev"),
				ref: url.searchParams.get("ref"),
				dir: url.searchParams.get("dir"),
				submodules: url.searchParams.get("submodules") === "true",
			};

			const root = await nix.evaluate(
				`
				let
					info = builtins.fromJSON ''${JSON.stringify(info)}'';
				in
					builtins.fetchGit (
						{ url = info.url; }
						// (if info.rev != null then { rev = info.rev; } else {})
						// (if info.ref != null then { ref = info.ref; } else {})
						// (if info.submodules != null then { submodules = info.submodules; } else {})
					)
			`,
				{
					impure: true,
					json: true,
				},
			);

			let storePath = await nix.realise(root);

			if (info.dir) {
				storePath = path.join(storePath, info.dir);
			}

			return {
				source: Source.Git,
				...info,
				path: storePath,
			};
		}
		case "github:": {
			const [owner, repo] = url.pathname.split("/");

			const repoResponse = await fetch(
				`https://api.github.com/repos/${owner}/${repo}`,
			);

			if (!repoResponse.ok) {
				log.fatal(
					`GitHub API error: ${repoResponse.status} ${repoResponse.statusText}`,
				);
				process.exit(1);
			}

			const { default_branch } = await repoResponse.json();

			const commitsResponse = await fetch(
				`https://api.github.com/repos/${owner}/${repo}/commits/${default_branch}`,
			);

			if (!commitsResponse.ok) {
				log.fatal(
					`GitHub API error: ${commitsResponse.status} ${commitsResponse.statusText}`,
				);
				process.exit(1);
			}

			const { sha } = await commitsResponse.json();

			const info = {
				owner,
				repo,
				rev: url.searchParams.get("rev") ?? sha,
				dir: url.searchParams.get("dir"),
				host: url.searchParams.get("host") ?? "github.com",
			};

			const root = await nix.evaluate(
				`
				builtins.fetchTarball {
					url = "https://api.${info.host}/repos/${info.owner}/${info.repo}/tarball/${info.rev}";
				}
			`,
				{
					impure: true,
					json: true,
				},
			);

			let storePath = await nix.realise(root);

			if (info.dir) {
				storePath = path.join(storePath, info.dir);
			}

			return {
				source: Source.Github,
				...info,
				path: storePath,
			};
		}
		case "gitlab:": {
			const [owner, repo] = url.pathname.split("/");

			const repoResponse = await fetch(
				`https://${url.searchParams.get("host") ?? "gitlab.com"}/api/v4/projects/${owner}%2F${repo}`,
			);

			if (!repoResponse.ok) {
				log.fatal(
					`GitLab API error: ${repoResponse.status} ${repoResponse.statusText}`,
				);
				process.exit(1);
			}

			const { default_branch } = await repoResponse.json();

			const commitsResponse = await fetch(
				`https://${url.searchParams.get("host") ?? "gitlab.com"}/api/v4/projects/${owner}%2F${repo}/repository/commits/${default_branch}`,
			);

			if (!commitsResponse.ok) {
				log.fatal(
					`GitLab API error: ${commitsResponse.status} ${commitsResponse.statusText}`,
				);
				process.exit(1);
			}

			const { id } = await commitsResponse.json();

			const info = {
				owner,
				repo,
				rev: url.searchParams.get("rev") ?? id,
				dir: url.searchParams.get("dir"),
				host: url.searchParams.get("host") ?? "gitlab.com",
			};

			const root = await nix.evaluate(
				`
				builtins.fetchTarball {
					url = "https://${info.host}/api/v4/projects/${info.owner}%2F${info.repo}/repository/archive.tar.gz?sha=${info.rev}";
				}
			`,
				{
					impure: true,
					json: true,
				},
			);

			let storePath = await nix.realise(root);

			if (info.dir) {
				storePath = path.join(storePath, info.dir);
			}

			return {
				source: Source.Gitlab,
				...info,
				path: storePath,
			};
		}
		case "sourcehut:": {
			const [owner, repo] = url.pathname.split("/");

			log.fatal("Sourcehut support is not yet implemented.");
			process.exit(1);

			return {
				source: Source.Sourcehut,
				owner,
				repo,
				rev: url.searchParams.get("rev"),
				dir: url.searchParams.get("dir"),
			};
		}
		case "tarball:": {
			let tarball = url.pathname;

			if (!tarball.startsWith("http://") && !tarball.startsWith("https://")) {
				tarball = `http://${tarball}`;
			}

			if (!isUrl(tarball)) {
				log.fatal(`Invalid tarball URL: ${tarball}`);
				process.exit(1);
			}

			const root = await nix.evaluate(
				`
				builtins.fetchTarball {
					url = "${tarball}";
				}
			`,
				{
					impure: true,
					json: true,
				},
			);

			const storePath = await nix.realise(root);

			return {
				source: Source.Tarball,
				url: tarball,
				path: storePath,
			};
		}
		case "http:":
		case "https:": {
			const tarball = url.href;

			const root = await nix.evaluate(
				`
				builtins.fetchTarball {
					url = "${tarball}";
				}
			`,
				{
					impure: true,
					json: true,
				},
			);

			const storePath = await nix.realise(root);

			return {
				source: Source.Tarball,
				url: tarball,
				path: storePath,
			};
		}
		default: {
			log.fatal(`Unsupported URI protocol: ${url.protocol}`);
			process.exit(1);
		}
	}
};
