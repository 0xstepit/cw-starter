START_DIR=$(pwd)

echo "🖊 Generating schema...!"
for c in ./contracts/*
do
cd "$c"
CMD="cargo run --example schema"

# discard output

eval $CMD > /dev/null

# remove redundant schemas

rm -rf ./schema/raw
cd "$START_DIR"
done
echo "✅ Schemas generated."