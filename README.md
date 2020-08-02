# monitor_config

Dependencies
============
  *Rust/Cargo
  *make

Introduction
============
A script to load monitor configuration file for gnome. Configuration files are saved at ~/.config/monitors.xml
Examples
--------
https://github.com/purple-phoenix/configs/tree/master/fedora/monitors


Configuration
============
MONITOR_CONFIG_DIR represents the directory where your monitor configuration files are stored.
This is configured once per install, so to change re-install monitor_config with a different value as described below


Installation
============
1. git clone https://github.com/purple-phoenix/monitor_config.git
2. cd monitor_config
3. make install MONITOR_CONFIG_DIR=xxxx


Usage
=====
Once installed run
monitors <regex>
The script looks in the MONITOR_CONFIG_DIR and loads the first configuration file found matching the given regular expression.
Depending on your regular expression you may need to surround the parameter in quotations. 
