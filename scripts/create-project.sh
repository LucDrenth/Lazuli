#!/bin/bash

COLOR_RED='\033[1;31m'
COLOR_CYAN='\033[1;36m'
COLOR_RESET='\033[0m'

NEW_PROJECT_NAME=$1
LAZULI_DIRECTORY=$(realpath "$(dirname "$0")/..")
TARGET_DIRECTORY=$(realpath "$(dirname "$LAZULI_DIRECTORY")")/$NEW_PROJECT_NAME

# Validate the given project name
if [ -z "$NEW_PROJECT_NAME" ]; then
    printf "\n${COLOR_RED}Please specify a project name ${COLOR_RESET}\n\n"
    exit 1;
fi

if [ -d "$TARGET_DIRECTORY" ]; then
    printf "\n${COLOR_RED}Project directory for '$NEW_PROJECT_NAME' already exists: $TARGET_DIRECTORY ${COLOR_RESET}\n\n"
    exit 1;
fi

# Step in to the directory where we will create the project
cd $LAZULI_DIRECTORY/..

# Create our new project
printf "\n${COLOR_CYAN}Creating new Lazuli project: $NEW_PROJECT_NAME ${COLOR_RESET}\n\n"
cargo new $NEW_PROJECT_NAME

# Step in to the newly created project
cd ./$NEW_PROJECT_NAME

# Add Lazuli as a dependency
printf "\n${COLOR_CYAN}Adding Lazuli dependency ${COLOR_RESET}\n\n"
cargo add lazuli --path ../lazuli

# Copy over assets from Lazuli to our new project
printf "\n${COLOR_CYAN}Copying over Lazuli assets ${COLOR_RESET}\n\n"

printf "engine assets\n"
cp -r ${LAZULI_DIRECTORY}/assets ./

printf "vscode settings\n"
cp -r ${LAZULI_DIRECTORY}/.vscode ./

printf "run script\n"
mkdir scripts && cp ${LAZULI_DIRECTORY}/scripts/run.sh ./scripts/

printf ".gitignore\n"
cp ${LAZULI_DIRECTORY}/project-template/gitignore-template ./.gitignore

printf "src folder\n"
cp -r ${LAZULI_DIRECTORY}/project-template/src ./

printf "README.md\n"
cp ${LAZULI_DIRECTORY}/project-template/README.md ./

# Getting here means we successfully created the project
printf "\n${COLOR_CYAN}Successfully created project '${NEW_PROJECT_NAME}' ${COLOR_RESET}\n\n"

# If we are running on macOS, open the newly created project in the finder
if [[ "$OSTYPE" == "darwin"* ]]; then
    open $TARGET_DIRECTORY
fi

exit 0
