{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.betterlogi;
in
{
  options.programs.betterlogi = {
    enable = lib.mkEnableOption "BetterLogi, a local-first Logitech device manager";

    package = lib.mkPackageOption pkgs "betterlogi" { };

    launchAtLogin = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Whether to start the BetterLogi agent with graphical sessions.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
    services.udev.packages = [ cfg.package ];

    systemd.user.services.betterlogi-agent = {
      description = "BetterLogi background agent";
      wantedBy = lib.optionals cfg.launchAtLogin [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      partOf = lib.optionals cfg.launchAtLogin [ "graphical-session.target" ];

      serviceConfig = {
        ExecStart = lib.getExe' cfg.package "betterlogi-agent";
        Restart = "on-failure";
        RestartSec = 5;
      };
    };
  };
}
