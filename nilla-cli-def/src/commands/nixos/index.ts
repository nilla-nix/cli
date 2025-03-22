import process from "node:process";
import path from "node:path";
import fs from "node:fs/promises";
import os from "node:os";

import arg from "arg";
import ora from "ora";

import log from "~/util/log";
import exec from "~/util/exec";
import spawn from "~/util/spawn";
import * as nix from "~/util/nix";

import help from "./help";
import spec from "./args";
import { resolve, exists } from "~/util/project";
import type { Command } from "~/commands";

export {
	help
};

const WARNING_REGEX = /warning:([^\n]*)/i;

export const run: Command["run"] = async () => {
	const args = arg(spec);

	if (args["--help"]) {
		help();
		process.exit(0);
	}

	if (args._.length < 2) {
		log.fatal("No subcommand specified.");
		process.exit(1);
	}

	if (args._.length > 3) {
		log.fatal("Only one system may be managed at once.");
		process.exit(1);
	}

	const subcommand = args._[1];
	const name = args._[2] ?? os.hostname();

	if (
		![
			"build",
			"build-vm",
			"switch",
			"boot",
			"test",
		].includes(subcommand)
	) {
		log.fatal(`Subcommand "${subcommand}" not found.`);
		process.exit(1);
	}

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

	let attribute = `config.systems.nixos.${name}.build`;

	await nix.existsInProject(entry, attribute)

	let output = "";
	let warnings: Array<string> = [];
	let errors: Array<string> = [];

	const spinner = ora("Running system build...").start();

	const code = await spawn("nixos-rebuild-ng", [
		subcommand,
		"--file",
		entry,
		"--attr",
		attribute,
		"--no-reexec",
	], {
		stdio: ["inherit", "pipe", "pipe"],
		onData: data => {
			output = data.toString();
			spinner.text = output;
		},
		onError: data => {
			const text = data.toString();
			spinner.text = text;
			errors.push(text);

			if (WARNING_REGEX.test(text)) {
				const warning = WARNING_REGEX.exec(text)![1];
				warnings.push(warning.trim());
			}
		},
	});

	if (code !== 0) {
		spinner.fail();
		for (const error of errors) {
			log.error(error);
		}
		log.fatal("nixos-rebuild-ng failed.");
		process.exit(1);
	}

	if (!output) {
		spinner.fail();
		log.fatal("No output received from nixos-rebuild-ng.");
		process.exit(1);
	}

	spinner.succeed("Complete!")

	if (warnings.length > 0) {
		console.log("");

		for (const warning of warnings) {
			log.warn(warning);
		}
	}
};
