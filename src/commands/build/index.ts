import process from "node:process";
import path from "node:path";
import fs from "node:fs/promises";

import arg from "arg";
import ora from "ora";

import log from "~/util/log";
import exec from "~/util/exec";
import * as nix from "~/util/nix";

import help from "./help";
import spec from "./args";
import { resolve, exists } from "~/util/project";
import type { Command } from "~/commands";

export {
	help
};

export const run: Command["run"] = async () => {
	const args = arg(spec);

	if (args["--help"]) {
		help();
		process.exit(0);
	}

	if (args._.length < 2) {
		log.fatal("No package specified.");
		process.exit(1);
	}

	if (args._.length > 2) {
		log.fatal("Only one package can be built at a time.");
		process.exit(1);
	}

	let name = args._[1] ?? "default";

	log.debug(`Resolving project: ${args["--project"] ?? process.cwd()}`);

	const project = args["--project"] ?
		await resolve(args["--project"])
		: await resolve(process.cwd());

	log.debug(`Resolved project: ${project.path!}`);

	const entry = path.join(project.path!, "nilla.nix");

	if (!await exists(entry)) {
		log.fatal(`No Nilla project found in "${project.path}".`);
		process.exit(1);
	}

	const system = await nix.system();

	let attribute = name;

	const spinner = ora({
		color: "white",
		prefixText: "🔨 ",
	});

	if (!name.includes(".")) {
		attribute = `config.packages.${name}.build.${system}`;
		spinner.text = `Building package "${name}".`;
	} else {
		attribute = name;

		if (!attribute.startsWith("config.")) {
			attribute = `config.${attribute}`;
		}

		spinner.text = `Building attribute "${name}".`;
	}

	spinner.start();

	await nix.build(entry, attribute, {
		link: true,
	});

	spinner.succeed("Package built.");
};
