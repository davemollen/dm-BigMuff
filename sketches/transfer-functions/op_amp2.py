from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the sustain value to see the difference in the frequency response
# Keep it between 0 and 1
sustain = 0.5

def generate_s_domain_coefficients(sustain):
  # The following transfer function was derived with QsapecNG:
  # ( C3 * R4 * R5 + 1-sus * C3 * R4 + C3 * R3 * R5 + 1-sus * C3 * R3 ) * s + ( R5 + 1-sus + sus + R4 + R3 )
  # ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( C1 * C2 * C3 * R1 * R2 * R4 * R5 + 1-sus * C1 * C2 * C3 * R1 * R2 * R4 + C1 * C2 * C3 * R1 * R2 * R4 * sus ) * s^3 + ( C1 * C2 * R1 * R2 * R5 + 1-sus * C1 * C2 * R1 * R2 + C1 * C2 * R1 * R2 * sus + C1 * C2 * R1 * R2 * R4 + C2 * C3 * R1 * R4 * R5 + C2 * C3 * R2 * R4 * R5 + 1-sus * C2 * C3 * R1 * R4 + 1-sus * C2 * C3 * R2 * R4 + C2 * C3 * R1 * R4 * sus + C2 * C3 * R2 * R4 * sus + C1 * C2 * R1 * R2 * R3 + ( - C1 * C3 * R1 * R3 * R5 ) + ( - 1-sus * C1 * C3 * R1 * R3 ) + ( - C1 * C3 * R1 * R3 * sus ) ) * s^2 + ( C2 * R1 * R5 + C2 * R2 * R5 + 1-sus * C2 * R1 + 1-sus * C2 * R2 + C2 * R1 * sus + C2 * R2 * sus + C2 * R1 * R4 + C2 * R2 * R4 + C3 * R4 * R5 + 1-sus * C3 * R4 + C3 * R4 * sus + C2 * R1 * R3 + C2 * R2 * R3 ) * s + ( R5 + 1-sus + sus + R4 + R3 )

  # This function implements this transfer function, but with less repeated calculations.

  r1 = 10000
  r2 = 47000
  r3 = 560000
  r4 = 62000
  r5 = 47
  r_sus_a = sustain * 10000
  r_sus_b = (1-sustain) * 10000
  c1 = 4.7e-9
  c2 = 1e-8
  c3 = 1e-5

  c3r4 = c3 * r4
  c3r3 = c3 * r3
  c2r2 = c2 * r2
  c2r1 = c2 * r1
  c1c2r1r2 = c1 * c2r1 * r2
  c1c2c3r1r2r4 = c1c2r1r2 * c3r4
  c1c3r1r3 = c1 * r1 * c3r3
  c2c3r1r4 = c2r1 * c3r4
  c2c3r2r4 = c2r2 * c3r4

  b0 = c3r4 * r5 + c3r4 * r_sus_a + c3r3 * r5 + c3r3 * r_sus_a
  b1 = r5 + r_sus_a + r_sus_b + r4 + r3

  a0 = c1c2c3r1r2r4 * r5 + c1c2c3r1r2r4 * r_sus_a + c1c2c3r1r2r4 * r_sus_b
  a1 = c1c2r1r2 * r5 + c1c2r1r2 * r_sus_a + c1c2r1r2 * r_sus_b + c1c2r1r2 * r4 + c2c3r1r4 * r5 + c2c3r2r4 * r5 + r_sus_a * c2c3r1r4 + r_sus_a * c2c3r2r4 + c2c3r1r4 * r_sus_b + c2c3r2r4 * r_sus_b + c1c2r1r2 * r3 - c1c3r1r3 * r5 - c1c3r1r3 * r_sus_a - c1c3r1r3 * r_sus_b
  a2 = c2r1 * r5 + c2r2 * r5 + c2r1 * r_sus_a + c2r2 * r_sus_a + c2r1 * r_sus_b + c2r2 * r_sus_b + c2r1 * r4 + c2r2 * r4 + c2r1 * r3 + c2r2 * r3 + c3r4 * r5 + r_sus_a * c3r4 + c3r4 * r_sus_b
  
  return ([0, 0, b0, b1], [a0, a1, a2, b1])

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
plt.ylim([-60, 25])
plt.show()