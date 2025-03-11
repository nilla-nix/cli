import arg from "arg";
import littlelog, {
	configure,
	LogLevel,
	LogLevelAsNumber,
	parseLogLevelNumber,
} from "@littlethings/log";

const args = arg(
	{
		"--verbose": arg.COUNT,
		"-v": "--verbose",
	},
	{
		permissive: true,
	},
);

if (args["--verbose"]) {
	const level =
		args["--verbose"] > 2
			? LogLevel.Trace
			: parseLogLevelNumber(args["--verbose"] as LogLevelAsNumber);

	configure({
		level,
	});
}

export default littlelog;
