import fs from "node:fs/promises";
import path from "node:path";

import exec from "~/util/exec";
import spawn from "~/util/spawn";

const escapeNixExpression = (code: string) => code.replaceAll(/'/g, "'\\''");

export const evaluate = async (
	code: string,
	{
		json = false,
		impure = false,
	}: {
		json?: boolean;
		impure?: boolean;
	} = {
		json: false,
		impure: false,
	},
) => {
	const expression = escapeNixExpression(code);

	const command = `nix eval --show-trace ${json ? "--json" : ""} ${impure ? "--impure" : ""} --expr '${expression}'`;

	const output = await exec(command);

	return json ? JSON.parse(output) : output;
};

export const system = async () => {
	const output = await evaluate("builtins.currentSystem", {
		impure: true,
		json: true,
	});

	return output;
};

export const realise = async (path: string) => {
	const output = await exec(`nix-store --realise ${path}`);

	return output.trim();
};

export const build = async (
	file: string,
	name: string,
	{
		link = false,
		report = true,
	}: {
		link?: boolean;
		report?: boolean;
	} = {
		link: false,
	},
) => {
	const command = `nix build ${link || report ? "" : "--no-link"} -f ${file} '${name}'`;

	const output = await exec(command);

	if (report) {
		// HACK: In the future we can probably make use of `--print-out-links` or similar
		// functionality to get the output path directly from the build command.
		const file = path.resolve(process.cwd(), "result");
		const target = await fs.readlink(file);

		if (!link) {
			await fs.unlink(file);
		}

		return target;
	}

	return output.trim();
};

export const shell = async (file: string, name: string) => {
	try {
		// TODO: We may want to utilize the `kexec` package to actually swap the process out with
		// the new development shell.
		await spawn("nix-shell", [file, "-A", name]);
	} catch (error) {
		throw error;
	}
};

export const getMainProgram = async (file: string, name: string) => {
	let short = name;

	if (name.includes(".")) {
		const parts = name.split(".");

		if (parts.length < 3) {
			short = parts[parts.length - 1];
		} else {
			short = parts[parts.length - 3];
		}
	}

	const main = await evaluate(
		`
		let
			project = import (builtins.toPath "${file}");
			system = "${await system()}";
		in
			project.${name}.meta.mainProgram or "${short}"
	`,
		{
			impure: true,
			json: true,
		},
	);

	return main;
};

export const existsInProject = async (file: string, name: string) => {
	let code;

	if (name.includes(".")) {
		const parts = name.split(".");
		const last = parts[parts.length - 1];
		const init = parts.slice(0, parts.length - 1).join(".");

		code = `
			let
				project = import (builtins.toPath "${file}");
			in
				(project.${init} or {}) ? ${last}
		`;
	} else {
		code = `
			let
				project = import (builtins.toPath "${file}");
			in
				project ? ${name}
		`;
	}

	const result = await evaluate(code, {
		impure: true,
		json: true,
	});

	return result;
};
