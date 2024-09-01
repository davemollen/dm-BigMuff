from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Get generated s-domain coefficients
num, den = ([-5.8928571428571, 0], [1, 119.04761904762])

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
plt.ylim([8.4, 16.1])
plt.show()