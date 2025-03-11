import cp from "node:child_process";
import { promisify } from "node:util";
import process from "node:process";

import log from "~/util/log";

const exec = promisify(cp.exec);

const SOURCE_REGEX = /error: hash mismatch in file downloaded from '([^']+)':/;
const FIXED_SOURCE_REGEX =
	/error: hash mismatch in fixed-output derivation '([^']+)':/;
const SPECIFIED_REGEX = /specified:\s+([^\n]+)/;
const GOT_REGEX = /got:\s+([^\n]+)/;

const run = async (command: string) => {
	try {
		const { stdout } = await exec(command);

		return stdout;
	} catch (error) {
		if (error instanceof Error) {
			if (SOURCE_REGEX.test(error.message)) {
				return handleHashMismatchError(error);
			}

			if (FIXED_SOURCE_REGEX.test(error.message)) {
				return handleFixedHashMismatchError(error);
			}
		}

		throw error;
	}
};

const handleHashMismatchError = async (error: Error) => {
	const source = SOURCE_REGEX.exec(error.message)?.[1];
	const specified = SPECIFIED_REGEX.exec(error.message)?.[1];
	const got = GOT_REGEX.exec(error.message)?.[1];

	if (!specified || !got || !source) {
		throw error;
	}

	console.log("");

	log.error("Hash mismatch detected for the following source:");
	log.error("");
	log.error(`${source}`);
	log.error("");
	log.error(`Current:  ${specified}`);
	log.error(`Expected: ${got}`);
	log.error("");
	log.fatal(
		"Please update your code to use the expected hash of the fetched source and try again.",
	);
	process.exit(1);
};

const handleFixedHashMismatchError = async (error: Error) => {
	const source = FIXED_SOURCE_REGEX.exec(error.message)?.[1];
	const specified = SPECIFIED_REGEX.exec(error.message)?.[1];
	const got = GOT_REGEX.exec(error.message)?.[1];

	if (!specified || !got || !source) {
		throw error;
	}

	console.log("");

	log.error(
		"Hash mismatch detected for the following fixed-output derivation:",
	);
	log.error("");
	log.error(`${source}`);
	log.error("");
	log.error(`Current:  ${specified}`);
	log.error(`Expected: ${got}`);
	log.error("");
	log.fatal(
		"Please update your code to use the expected hash of the fixed-output derivation and try again.",
	);
	process.exit(1);
};

export default run;
