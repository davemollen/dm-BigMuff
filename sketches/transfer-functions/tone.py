from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tone value to see the difference in the frequency response
# Keep it between 0 and 1
tone = 0.5

def generate_s_domain_coefficients(tone):
  # The following transfer function was derived with QsapecNG:
  # 1-t * C1 * C2 * C3 * R1 * R4 * s^3 + ( C1 * C2 * R1 * R4 + 1-t * C1 * C2 * R1 + C1 * C2 * R1 * t + 1-t * C2 * C3 * R1 ) * s^2 + ( C1 * t + C1 * R1 + C2 * R1 ) * s
  # -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( 1-t * C1 * C2 * C3 * R1 * R4 + C1 * C2 * C3 * R1 * R4 * t ) * s^3 + ( C1 * C2 * R1 * R4 + 1-t * C1 * C2 * R1 + C1 * C2 * R1 * t + 1-t * C1 * C3 * R4 + C1 * C3 * R4 * t + C1 * C3 * R1 * R4 + 1-t * C2 * C3 * R1 + C2 * C3 * R1 * t ) * s^2 + ( C1 * R4 + 1-t * C1 + C1 * t + C1 * R1 + C2 * R1 + 1-t * C3 + C3 * t + C3 * R1 ) * s + 1

  # This function implements this transfer function, but with less repeated calculations.
  c1 = 1e-6
  c2 = 1e-7
  c3 = 1.2e-7
  r1 = 1200
  r2_a = (1-tone) * 10000
  r2_b = tone * 10000 
  r4 = 5600

  c1r1 = c1 * r1
  c1c2r1 = c1r1 * c2
  c3r4 = c3 * r4
  c3r1 = c3 * r1
  c1c3r4 = c1 * c3r4
  c1c2c3r1r4 = c1c2r1 * c3r4
  c1c2c3r1r4r2_a = c1c2c3r1r4 * r2_a
  b0 = c1c2c3r1r4 * r2_b

  b1 = c1c2r1 * r4 + c1c2r1 * r2_b + c1c2r1 * r2_a + r2_b * c2 * c3r1
  b2 = c1 * r2_a + c1r1 + c2 * r1
  
  a0 = b0 + c1c2c3r1r4r2_a
  a1 = b1 + c1c3r4 * r2_b + c1c3r4 * r2_a + c1c3r4 * r1 + c2 * c3r1 * r2_a
  a2 = b2 + c1 * r4 + c1 * r2_b + c3 * r2_b + c3 * r2_a + c3r1

  return ([b0, b1, b2, 0], [a0, a1, a2, 1])


# Get generated s-domain coefficients
num, den = generate_s_domain_coefficients(tone)
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
plt.ylim([-36, 0])
plt.show()