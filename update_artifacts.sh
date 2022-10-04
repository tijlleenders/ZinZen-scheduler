#!/bin/bash
echo Checking if jq is installed...
if ! command -v jq &> /dev/null
then
    echo "Installing jq - needed for parsing json..."
    sudo apt-get -y install jq > /dev/null
fi

last_artifact_id=$(cat artifact_id.txt)
echo Last artifact id is $last_artifact_id
echo Fetching latest artifact id from ZinZen Scheduler repo...
token=$(echo $ZINZENTOKEN)
current_artifact_id=$(curl -s \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $token" \
  https://api.github.com/repos/tijlleenders/ZinZen-scheduler/actions/artifacts | jq ".artifacts[0].id")
echo Received artifact id: $current_artifact_id
if [ $current_artifact_id = $last_artifact_id ]
then
    echo No new build found.
else
    echo New build detected.

echo Requesting download url...
line=$(curl -s -H "Accept: application/vnd.github+json" -H "Authorization: Bearer $token" https://api.github.com/repos/tijlleenders/zinzen-scheduler/actions/artifacts/$current_artifact_id/zip -I | grep location) &> /dev/null
download_url=${line:10}
echo Successfully received download url

echo Using wget to download artifacts...
wget -O wasm-build-pkg.zip $download_url > /dev/null
echo Unzipping into pkg directory...
unzip -o wasm-build-pkg.zip -d pkg
echo $current_artifact_id > artifact_id.txt

echo Cleaning up...
rm wasm-build-pkg.zip
echo Ok
fi
