import process from "node:process";
import log from "~/util/log";

export interface Command {
	run: () => Promise<void>;
	help: () => void;
}

export const get = async (name: string) => {
	const glob = import.meta.glob("./*/index.ts");

	const match = `./${name}/index.ts`;

	if (!glob[match]) {
		log.fatal(`Command "${name}" not found.`);
		process.exit(1);
	}

	const command = (await glob[match]()) as Command;

	return command;
};
