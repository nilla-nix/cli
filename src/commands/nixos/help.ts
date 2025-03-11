import kleur from "kleur";
import sources from "~/util/help/sources";
import common from "~/util/help/common";
import project from "~/util/help/project";

export default function help() {
	// prettier-ignore
	const message = `
${kleur.bold("🍦 Nilla - nixos")}

${kleur.bold("DESCRIPTION")}

  Manage a NixOS system in a Nilla project.

${kleur.bold("USAGE")}

  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos <subcommand> [options]

${kleur.bold("SUBCOMMANDS")}

  switch [name]             Build and switch to a NixOS system
  build [name]              Build a NixOS system
  build-vm [name]           Build a NixOS system VM
  boot [name]               Add a boot entry for a NixOS system
  test [name]               Activate a NixOS system temporarily

${kleur.bold("OPTIONS")}

${project}

${common}

${kleur.bold("EXAMPLES")}

  ${kleur.dim("# Switch to a NixOS configuration in a local Nilla project.")}
  ${kleur.dim("# By default, subcommands target the system matching your computer's hostname.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos switch

  ${kleur.dim("# Build a NixOS configuration in a local Nilla project.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos build

  ${kleur.dim("# Build a NixOS VM in a Nilla project on GitHub.")}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos build-vm --project github:myuser/myrepo

  ${kleur.dim(`# Add a boot entry for the "mysystem" NixOS configuration in a Nilla project.`)}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos boot mysystem

  ${kleur.dim(`# Temporarily activate a NixOS configuration in a Nilla project.`)}
  ${kleur.dim("$")} ${kleur.bold("nilla")} nixos test

${sources}
	`;

	console.log(message);
}
