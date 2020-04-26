{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.dsc;
  pkgDesc = "dsc";

  confFile = pkgs.writeText "dsc_config.json" ''
    {
    "database": {
      "mode": "None",
      "path": "/Users/firebug/Desktop/dsc_data"
    },
    "line": {
      "id": ${toString cfg.line.number},
      "name": "Linie ${toString cfg.line.number}",
      "short_name": "${toString cfg.line.number}"
    },
    "default_discipline": "lg_demo",
    "websocket": {
      "url": "${cfg.websocket.address}:${cfg.websocket.port}"
    }
  }
  '';

in {

  options = {

    services.dsc = {

      enable = mkEnableOption "dsc";

      user = mkOption {
        type = types.str;
        default = "dsc";
        description = "dsc user";
      };

      group = mkOption {
        type = types.str;
        default = "dsc";
        description = "dsc group";
      };

      stateDir = mkOption {
        type = types.path;
        default = "/var/lib/dsc/";
        description = "state directory for dsc";
        example = "/home/bob/.dsc/";
      };

      websocket = {
        address = mkOption {
          type = types.str;
          default = "0.0.0.0";
          description = "websocket address to listen to";
        };
        port = mkOption {
          type = types.str;
          default = "3009";
          description = "websocket port to listen to";
        };
      };

      line = {
        number = mkOption {
          type = types.int;
          default = 0;
          description = "dsc line number";
        };
      };


    };

  };

  config = mkIf cfg.enable {

    users.users.${cfg.user} = {
      name = cfg.user;
      group = cfg.group;
      extraGroups = [ "dialout" ];
      home = cfg.stateDir;
      createHome = true;
      description = pkgDesc;
    };

    users.groups.${cfg.user} = {
      name = cfg.group;
    };

    systemd.services.dsc = {
      description = pkgDesc;
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;
        Restart = "always";
	WorkingDirectory = cfg.stateDir;
        ExecStart = ''
          ${pkgs.dsc}/bin/dsc -c ${confFile} -m ${pkgs.dsc}/modes/
        '';
      };
    };

  };

}
