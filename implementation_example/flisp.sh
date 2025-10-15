#!/bin/sh -

# flisp.sh, an example interpreter for running flisp programs

# Dependencies:
#  · sh(1p)
#  · sort(1p)

E_SUCCESS=0
E_USER=1
E_EXEC=2
E_INTERNAL=13
readonly E_USER

die()
{
	retval=$(($1))
	{
		# shellcheck disable=SC2059
		printf "$@"
		printf '\n'
		if [ $retval -eq $E_USER ]; then
			printf 'Run with -h for help.\n'
		fi
	} >&2
	exit $retval
}

print_help()
{
	cat <<- EOF
	${0##*/} - run flisp routines in the current directory

	Usage: ${0##*/} -h
	       ${0##*/} [[OPTIONS] --] [ARGS...]

	Options:
	  -h      Display this help message and exit.
	  -r DIR  Use DIR as runtime directory for managing arguments, subprocesses
	          and return values. If not set, \$XDG_RUNTIME_DIR/flisp/\$PID is
	          used.
	  --      Treat the remainder as positional arguments.
	EOF
}

invoke_call()
{
	# A 'function' runtime directory has the following structure:
	# dir/
	#  - meta/
	#     - arg1
	#     - arg2
	#     - ...
	#     - return
	#  - data/
	#     - ... (runtime directory for function itself)
	mkdir "$RUNDIR"/"$1"
	mkdir "$RUNDIR"/"$1"/meta
}

CWD=$(pwd) || die $E_INTERNAL 'Unable to determine current working directory'
readonly CWD

# Optional arguments:
RUNDIR=
RUNDIR_SET=false
while getopts :hr:- opt; do
	case "$opt" in
		(h) print_help; exit $E_SUCCESS ;;
		(r) RUNDIR=$OPTARG; RUNDIR_SET=true ;;
		(-) break ;;
		(:) die $E_USER 'Missing argument for -%s' "$OPTARG" ;;
		('?') die $E_USER 'Unknown option -%s' "$OPTARG" ;;
		(*) die $E_INTERNAL 'Unhandled option -%s' "$OPTARG" ;;
	esac
done
shift $((OPTIND - 1))
unset OPTARG
readonly RUNDIR_SET

# Set up runtime directory if necessary:
if ! $RUNDIR_SET; then
	RUNDIR=$XDG_RUNTIME_DIR/flisp/$$
	mkdir "$RUNDIR" || die $E_EXEC 'Could not create runtime directory'
fi
readonly RUNDIR

# Execute operations:
for file in *; do
	operation=${file#*:}  # Strip leading order marker
	case "$operation" in
		(call) invoke_call "$file" "$@" ;;
		(*) die $E_EXEC 'Unknown operation %s/%s' "$CWD" "$operation" ;;
	esac
	# TODO: Handle return values
done
