mkdir -p tmp
printf "@a\nAAA\n+\nAAA\n@c\nCCC\n+\nCCC\n@t\nTTT\n+\nTTT\n" | gzip > tmp/test.fq.gz 
printf "a\nt\n" > tmp/id_list 
printf "@a\nAAA\n+\nAAA\n@t\nTTT\n+\nTTT\n" > tmp/expected.fq  
cargo run -- --in-fastq tmp/test.fq.gz --in-id-list tmp/id_list > tmp/out.fq 
diff tmp/expected.fq tmp/out.fq > /dev/null; echo "exit code: $?"
