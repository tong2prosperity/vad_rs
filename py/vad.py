from funasr import AutoModel

# model = AutoModel(model="fsmn-vad", model_revision="v2.0.4")

# wav_file = "/Users/trevorlink/Downloads/mumble.wav"

# res = model.generate(input=wav_file)

# print(res)



def export_model():
    model = AutoModel(model="fsmn-vad", model_revision="v2.0.4")
    res = model.export(quantize=False)


if __name__ == "__main__":
    export_model()