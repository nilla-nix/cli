import arg from "arg";

const spec: arg.Spec = {
	"--help": Boolean,
	"-h": "--help",

	"--verbose": arg.COUNT,
	"-v": "--verbose",

	"--version": Boolean,
};

export default spec;
