#!/usr/bin/env node

import arg from "arg";

import log from "~/util/log";
import spec from "~/args";
import help from "~/help";
import * as commands from "~/commands";
import pkg from "../package.json";

const main = async () => {
	const args = arg(spec, {
		permissive: true,
	});

	if (args["--help"] && args._.length === 0) {
		help();
		process.exit(0);
	}

	if (args["--version"]) {
		log.info(`Nilla CLI v${pkg.version}`);
		process.exit(0);
	}

	if (args._.length === 0) {
		log.fatal("No command specified.\n");
		help();
		process.exit(1);
	}

	const command = await commands.get(args._[0]);

	if (command) {
		await command.run();
	} else {
		log.fatal(`Command "${args._[0]}" not found.`);
		process.exit(1);
	}
};

main().catch(error => {
	console.log("");

	if (error instanceof Error) {
		const lines = error.toString().split("\n").slice(1);
		for (const line of lines) {
			log.fatal(line);
		}
	} else {
		log.fatal(error);
	}

	process.exit(1);
});
