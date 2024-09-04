from funasr_onnx import Fsmn_vad


model = Fsmn_vad("/Users/trevorlink/Project/tong2prosperity/rust_vad/py/")

res =  model("/Users/trevorlink/Downloads/mumble.wav")

print(res)