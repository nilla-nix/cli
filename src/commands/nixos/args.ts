import arg from "arg";
import root from "~/args";

const spec: arg.Spec = {
	...root,

	"--project": String,
	"-p": "--project",
};

export default spec;
