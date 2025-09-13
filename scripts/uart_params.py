import argparse

parser = argparse.ArgumentParser(prog='uart_params', description="calculate parameters for UART")
parser.add_argument("--freq", type=int, help="UART clock frequency")
parser.add_argument("--baud_rate", type=int, help="baud rate")

args = parser.parse_args()

freq = args.freq
baud_rate = args.baud_rate

baud_rate_divisor = freq/(16*baud_rate)

brdi = int(baud_rate_divisor)
brdf = baud_rate_divisor - brdi

m = int((brdf*64)+0.5)
baud_rate_divider = brdi+m/64

generated_baud_rate = int(freq/(16*baud_rate_divider))
error = (generated_baud_rate-baud_rate)/baud_rate*100

print(f"""
UART clock frequency: {freq}
baud rate: {baud_rate}
baud rate divisor: {freq} / (16 * {baud_rate}) = {baud_rate_divisor}
BRDI: {brdi}
BRDF: {brdf}
fractional part m: {m}
generated baud rate divider: {brdi} + ({m} / 64) = {baud_rate_divider}
generated baud rate: {freq} / (16 * {baud_rate_divider}) = {generated_baud_rate}
error: ({generated_baud_rate} - {baud_rate}) / {baud_rate} * 100 = {error}
""")
