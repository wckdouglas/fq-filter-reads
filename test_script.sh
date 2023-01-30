set -x

# mock data
mkdir -p tmp
printf "@a\nAAA\n+\nAAA\n@c\nCCC\n+\nCCC\n@t\nTTT\n+\nTTT\n" > tmp/test.fq 
cat tmp/test.fq | gzip > tmp/test.fq.gz 
printf "a\nt\n" > tmp/id_list 
printf "@a\nAAA\n+\nAAA\n@t\nTTT\n+\nTTT\n" > tmp/expected.fq  
printf "@c\nCCC\n+\nCCC\n" > tmp/expected_inverse.fq  

# run
cargo run -- --in-fastq tmp/test.fq.gz --in-id-list tmp/id_list > tmp/out.fq 
diff tmp/expected.fq tmp/out.fq > /dev/null; echo "exit code: $?"

cargo run -- --in-fastq tmp/test.fq --in-id-list tmp/id_list > tmp/out.fq 
diff tmp/expected.fq tmp/out.fq > /dev/null; echo "exit code: $?"

# run inverse
cargo run -- --in-fastq tmp/test.fq.gz --in-id-list tmp/id_list --inverse > tmp/out.fq 
diff tmp/expected_inverse.fq tmp/out.fq > /dev/null; echo "exit code: $?"
