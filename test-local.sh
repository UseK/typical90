cargo build --bin "$1"
# cargo build --bin "$1"
# shellcheck disable=SC2038
# find "./testcase/$1/in/" -type f -name "*.txt"
# shellcheck disable=SC2038
mkdir -p ./testcase/"$1"/my_ans
ls "./testcase/$1/in/" | xargs -I {} sh -c "./target/debug/$1 < ./testcase/$1/in/{} > ./testcase/$1/my_ans/{}; echo generated ./testcase/$1/my_ans/{}"
# find "./testcase/$1/in/" -type f -name "*.txt"| xargs -I {} sh -c "./target/debug/$1 < {} > ./testcase/$1/my_ans"
ls "./testcase/$1/in/" | xargs -I {} sh -c "echo Difference in {}; diff --strip-trailing-cr ./testcase/$1/my_ans/{} ./testcase/$1/out/{}; echo;"
