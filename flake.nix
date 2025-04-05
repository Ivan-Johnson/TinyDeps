{
	description = "itj_daemon_hello_world";

	inputs.nixpkgs.url = "nixpkgs/nixos-24.11-small";

	outputs = { self, nixpkgs }: let
		pkgs = import nixpkgs { system = "x86_64-linux"; };
		rustPlatform = pkgs.rustPlatform;
		itj_daemon_hello_world = rustPlatform.buildRustPackage {
			pname = "itj_daemon_hello_world";

			version = "0.1.0";

			src = ./.;

			# compile time environment variables
			# env = {
			#       # Works
			# 	ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME = "Alice";
			#       # Does not work as-is. Would need to do some heavy refactoring to be able to access `options` from here.
			# 	ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME = options.programs.itj_daemon_hello_world.default_server_name;
			# };

			buildInputs = [
				pkgs.makeWrapper
				# for itj_tiny_deps/src/ipc/base_impl/nc.rs
				pkgs.netcat
			];

			checkFlags = [
			];

			cargoLock.lockFile = ./Cargo.lock;

			postInstall = ''
				wrapProgram "$out/bin/itj_daemon_hello_world" --suffix PATH : "${pkgs.netcat}/bin"
				mv "$out/bin/itj_daemon_hello_world" "$out/bin/hello-world"
			'';
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

		packages.${pkgs.system}.default = itj_daemon_hello_world;

		nixos_options =
			{ config, lib, pkgs, ... }:

			let
				cfg = config.programs.itj_daemon_hello_world;
			in
			{
				options = {
					programs.itj_daemon_hello_world = {
						enable = lib.mkEnableOption "itj_daemon_hello_world";
						default_server_name = lib.mkOption {
							type = lib.types.str;
							description = "Default server name";
							default = "Server";
						};
					};
				};

				config = lib.mkMerge [
					(lib.mkIf cfg.enable {
						environment.systemPackages = [ itj_daemon_hello_world ];

						users.groups.itj_daemon_hello_world = {};

						users.users.itj_daemon_hello_world = {
							isSystemUser = true;
							description = "User for itj_daemon_hello_world";
							group = "itj_daemon_hello_world";
						};

						systemd.services."itj_daemon_hello_world" = {
							description = "itj_daemon_hello_world";

							path = [
								itj_daemon_hello_world
							];

							environment = {
								RUST_BACKTRACE = "1";
								ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME = cfg.default_server_name;
							};

							script = "hello-world daemon";

							wantedBy = [ "default.target" ];

							serviceConfig = {
								User="itj_daemon_hello_world";
								Group="itj_daemon_hello_world";
								Restart="always";
								RestartSec="20";
							};
						};
					})
				];
			};
	};
}
