import kleur from "kleur";
import project from "~/util/help/project";
import sources from "~/util/help/sources";
import common from "~/util/help/common";

export default function help() {
	// prettier-ignore
	const message = `
${kleur.bold("🍦 Nilla - shell")}

${kleur.bold("DESCRIPTION")}

  Start a development shell from a Nilla project.

${kleur.bold("USAGE")}

  ${kleur.dim("$")} ${kleur.bold("nilla")} shell <name> [options]

${kleur.bold("OPTIONS")}

${project}

${common}

${kleur.bold("EXAMPLES")}

  ${kleur.dim("# Start the default shell in a Nilla project on GitHub.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} shell --project github:myuser/myrepo

  ${kleur.dim("# Start a specific shell in a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} shell myshell

${sources}
	`;

	console.log(message);
}
