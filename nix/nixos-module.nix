{
  config,
  pkgs,
  lib,
  ...
}:
let
  cfg = config.services.pnl-calculator;
  pnl-calculator = pkgs.callPackage ./package.nix { };
in
{
  options = {
    services.pnl-calculator = {
      enable = lib.mkEnableOption "Enable the rust app";
    };
  };

  config = lib.mkIf cfg.enable {
    users.groups.pnl-calculator = { };
    users.users.pnl-calculator = {
      isSystemUser = true;
      group = "pnl-calculator";
    };

    systemd.services.pnl-calculator = {
      wantedBy = [ "multi-user.target" ];
      description = "Rust App.";
      after = [ "network.target" ];
      serviceConfig = {
        ExecStart = "${lib.getExe pnl-calculator}";
        User = "pnl-calculator";
        Group = "pnl-calculator";
      };
    };
  };
}
