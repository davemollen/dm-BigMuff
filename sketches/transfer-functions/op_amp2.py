from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the sustain value to see the difference in the frequency response
# Keep it between 0 and 1
sustain = 0.5
sustain *= sustain * sustain

def generate_s_domain_coefficients(sustain):
  # The following transfer function was derived with QsapecNG:
  # ( C1 * R2 * R5 + 1-sus * C1 * R2 + C1 * R1 * R5 + 1-sus * C1 * R1 ) * s + ( R5 + 1-sus + sustain + R2 + R1 )
  # ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( C1 * C2 * R2 * R5 * R6 + 1-sus * C1 * C2 * R2 * R6 + C1 * C2 * R2 * R6 * sustain ) * s^2 + ( C2 * R5 * R6 + 1-sus * C2 * R6 + C2 * R6 * sustain + C2 * R2 * R6 + C1 * R2 * R5 + 1-sus * C1 * R2 + C1 * R2 * sustain + C2 * R1 * R6 ) * s + ( R5 + 1-sus + sustain + R2 + R1 )

  # This function implements this transfer function, but with less repeated calculations.

  c1 = 1e-5
  c2 = 1e-8
  r1 = 560000
  r2 = 62000
  r3_a = (1-sustain) * 10000
  r3_b = sustain * 10000
  r5 = 47
  r6 = 47000

  c1r1 = c1 * r1
  c1r2 = c1 * r2
  c2r6 = c2 * r6
  c1c2r2r6 = c1r2 * c2r6
  
  b1_a1 = c1r2 * r5 + c1r2 * r3_b
  b1 = b1_a1 + c1r1 * r5 + c1r1 * r3_b
  b2 = r5 + r3_b + r3_a + r2 + r1
  a0 = c1c2r2r6 * r5 + c1c2r2r6 * r3_b + c1c2r2r6 * r3_a
  a1 = b1_a1 + c2r6 * r5 + c2r6 * r3_b + c2r6 * r3_a + c2r6 * r2 + c2r6 * r1 + c1r2 * r3_a
  
  return ([0., b1, b2], [a0, a1, b2])

# Get generated s-domain coefficients
num, den = generate_s_domain_coefficients(sustain)
print('s-domain coefficients', (num, den))

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
plt.ylim([-60, 20])
plt.show()