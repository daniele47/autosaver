# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_autosaver_global_optspecs
	string join \n nocolor= h/help V/version
end

function __fish_autosaver_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_autosaver_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_autosaver_using_subcommand
	set -l cmd (__fish_autosaver_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c autosaver -n "__fish_autosaver_needs_command" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_needs_command" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_needs_command" -s V -l version -d 'Print version'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "list" -d 'Show differences'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "save" -d 'Save to backup'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "restore" -d 'Restore from backup'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "rmhome" -d 'Delete from home'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "rmbackup" -d 'Delete from backup'
complete -c autosaver -n "__fish_autosaver_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c autosaver -n "__fish_autosaver_using_subcommand list" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_using_subcommand list" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_using_subcommand save" -s y -l assumeyes -r
complete -c autosaver -n "__fish_autosaver_using_subcommand save" -s n -l assumeno -r
complete -c autosaver -n "__fish_autosaver_using_subcommand save" -s a -l all -r
complete -c autosaver -n "__fish_autosaver_using_subcommand save" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_using_subcommand save" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_using_subcommand restore" -s y -l assumeyes -r
complete -c autosaver -n "__fish_autosaver_using_subcommand restore" -s n -l assumeno -r
complete -c autosaver -n "__fish_autosaver_using_subcommand restore" -s a -l all -r
complete -c autosaver -n "__fish_autosaver_using_subcommand restore" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_using_subcommand restore" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_using_subcommand rmhome" -s y -l assumeyes -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmhome" -s n -l assumeno -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmhome" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmhome" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_using_subcommand rmbackup" -s y -l assumeyes -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmbackup" -s n -l assumeno -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmbackup" -l nocolor -r
complete -c autosaver -n "__fish_autosaver_using_subcommand rmbackup" -s h -l help -d 'Print help'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "list" -d 'Show differences'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "save" -d 'Save to backup'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "restore" -d 'Restore from backup'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "rmhome" -d 'Delete from home'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "rmbackup" -d 'Delete from backup'
complete -c autosaver -n "__fish_autosaver_using_subcommand help; and not __fish_seen_subcommand_from list save restore rmhome rmbackup help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
