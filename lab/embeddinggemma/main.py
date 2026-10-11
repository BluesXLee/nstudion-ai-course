from sentence_transformers import SentenceTransformer

model = SentenceTransformer("google/embeddinggemma-2")

query = "What causes the northern lights?"
document = "The stock market fell today."

query_emb = model.encode(query, prompt_name="SearchQuery")
doc_emb = model.encode(document, prompt_name="Document")
print(model.similarity(query_emb, doc_emb))

documents = [
    "The northern lights are caused by charged particles from the sun.",
    "The stock market fell sharply today.",
    "Auroras appear near the polar regions where Earth's magnetic field funnels particles.",
    "Charged particles collide with gases in the atmosphere, producing colorful light.",
]

doc_embs = model.encode(documents, prompt_name="Document")
sims = model.similarity(query_emb, doc_embs)[0]

for doc, score in sorted(zip(documents, sims.tolist()), key=lambda x: -x[1]):
    print(f"{score:.4f}  {doc}")
