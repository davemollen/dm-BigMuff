from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Get generated s-domain coefficients
num, den = ([-813008.1300813, 0], [1, 14210.344231102, 368043.51746551])

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([16, 36])
plt.show()