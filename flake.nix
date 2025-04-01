{
	description = "TODO document this";

	inputs.nixpkgs.url = "nixpkgs/nixos-24.11-small";

	outputs = { self, nixpkgs }: let
		pkgs = import nixpkgs { system = "x86_64-linux"; };
		rustPlatform = pkgs.rustPlatform;
		itj_tiny_deps = rustPlatform.buildRustPackage {
			pname = "itj_tiny_deps";

			version = "0.1.0";

			src = ./.;

			buildInputs = [
			];

			checkFlags = [
				# Can be used to skip failing tests
				# "--skip=api::...::test_simple_5m"
			];

			cargoLock.lockFile = ./Cargo.lock;
		};
	in {
		devShells.${pkgs.system}.default = pkgs.mkShell {
			buildInputs = [
				pkgs.cargo
				pkgs.cargo-flamegraph
				pkgs.lldb
				pkgs.rustc
				pkgs.rustfmt
			];
		};

		packages.${pkgs.system}.default = itj_tiny_deps;

		nixos_options =
			{ config, lib, pkgs, ... }:

			let
				cfg = config.programs.itj_tiny_deps;
			in
			{
				options = {
					programs.itj_tiny_deps = {
						enable = lib.mkEnableOption "TODO document this";

						user_agent = lib.mkOption {
							type = lib.types.str;
							description = "Sample option";
						};
					};
				};

				config = lib.mkMerge [
					(lib.mkIf cfg.enable {
						environment.systemPackages = [ itj_tiny_deps ];
					})
				];
			};
	};
}
