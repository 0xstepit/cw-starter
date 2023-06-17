echo "🖊 Generating schema...!"
for c in ./contracts/*
do
  CMD="cargo run --example schema"
  # discard output
  eval $CMD > /dev/null
done
echo "✅ Schemas generated."