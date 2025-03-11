import { spawn } from "node:child_process";

const run = (
	program: string,
	args: Array<string>,
	{
		stdio = "inherit",
		onData,
		onError,
	}: {
		stdio?:
			| "inherit"
			| "pipe"
			| "ignore"
			| Array<"inherit" | "pipe" | "ignore">;
		onData?: (data: string) => void;
		onError?: (data: string) => void;
	} = {},
) =>
	new Promise<number | null>((resolve, reject) => {
		try {
			const process = spawn(program, args, {
				stdio: stdio,
			});

			process.on("exit", (code) => {
				resolve(code);
			});

			process.on("error", (error) => {
				reject(error);
			});

			if (onData) {
				process.stdout?.on("data", onData);
			}

			if (onError) {
				process.stderr?.on("data", onError);
			}
		} catch (error) {
			throw error;
		}
	});

export default run;
