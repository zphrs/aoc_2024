if [ $# -ne 1 ] || [ $1 = "--help" ];
then
echo "Usage: \n$ ./check-p1 answer";
exit 1;
fi

v=$(aoc submit -qd 4 1 $1);

script='
if [ $? -ne 0 ];
then
exit 1;
fi
echo "$v" | grep "not the right answer" > /dev/null;
wrongAnswer=$?;
echo "$v" | grep "You gave an answer too recently" > /dev/null;
tooRecent=$?;
echo "$v" | grep "seem to be solving the right level." > /dev/null;
alreadyCompleted=$?;
if [ $wrongAnswer -eq 0 ];
then
    echo "$v\\n";
    exit 1;
elif [ $tooRecent -eq 0 ]; 
then
    echo "$v\\n";
    exit 1;
elif [ $alreadyCompleted -eq 0 ]; 
then
    echo "$v\\n";
    exit 1;
fi
echo "$v\\n"'
# eval "$script";

aoc download -d 4 --overwrite;

echo '
if [ $# -ne 1 ] || [ $1 = "--help" ];
then
echo "Usage: \n$ ./check-p2 answer";
exit 1;
fi' > ./check-p2;
echo "v=\$(aoc submit -d 4 2 \$1)" >> ./check-p2;
echo "$script" >> ./check-p2;
echo "aoc download -d 4 --overwrite;" >> ./check-p2;
chmod +x ./check-p2;