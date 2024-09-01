from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tone value to see the difference in the frequency response
# Keep it between 0 and 1
tone = 1.

def generate_s_domain_coefficients(tone):
  # The following transfer function was derived with QsapecNG:
  # ( C2 * C3 * R2 * R3 * Rr + C2 * C3 * R2 * R3 * Rl + C2 * C3 * R2 * Rl * Rr ) * s^2 + ( C3 * R3 * Rr + C3 * R3 * Rl + C3 * Rl * Rr + C2 * R2 * Rr + C2 * R2 * Rl + C3 * R2 * Rr ) * s + ( Rr + Rl )
  # ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( C1 * C2 * C3 * R1 * R2 * R3 * Rr + C1 * C2 * C3 * R1 * R2 * R3 * Rl + C1 * C2 * C3 * R1 * R2 * Rl * Rr ) * s^3 + ( C2 * C3 * R2 * R3 * Rr + C2 * C3 * R2 * R3 * Rl + C2 * C3 * R2 * Rl * Rr + C1 * C3 * R1 * R3 * Rr + C1 * C3 * R1 * R3 * Rl + C1 * C3 * R1 * Rl * Rr + C1 * C2 * R1 * R2 * Rr + C1 * C2 * R1 * R2 * Rl + C2 * C3 * R1 * R2 * Rl ) * s^2 + ( C3 * R3 * Rr + C3 * R3 * Rl + C3 * Rl * Rr + C2 * R2 * Rr + C2 * R2 * Rl + C1 * R1 * Rr + C1 * R1 * Rl + C3 * R1 * Rl ) * s + ( Rr + Rl )

  # This function implements this transfer function, but with less repeated calculations.

  R1 = 10e3
  C1 = 1.8e-8
  R2 = 10e3
  C2 = 1e-8
  C3 = 2.7e-8
  R3 = 470

  Rl = 1. - tone
  Rr = tone

  b0 = C2 * C3 * R2 * R3 * Rr + C2 * C3 * R2 * R3 * Rl + C2 * C3 * R2 * Rl * Rr

  return (
    [
      b0,
      C3 * R3 * Rr + C3 * R3 * Rl + C3 * Rl * Rr + C2 * R2 * Rr + C2 * R2 * Rl + C3 * R2 * Rr,
      Rr + Rl
    ],
    [
      C1 * C2 * C3 * R1 * R2 * R3 * Rr + C1 * C2 * C3 * R1 * R2 * R3 * Rl + C1 * C2 * C3 * R1 * R2 * Rl * Rr,
      b0 + C1 * C3 * R1 * R3 * Rr + C1 * C3 * R1 * R3 * Rl + C1 * C3 * R1 * Rl * Rr + C1 * C2 * R1 * R2 * Rr + C1 * C2 * R1 * R2 * Rl + C2 * C3 * R1 * R2 * Rl,
      C3 * R3 * Rr + C3 * R3 * Rl + C3 * Rl * Rr + C2 * R2 * Rr + C2 * R2 * Rl + C1 * R1 * Rr + C1 * R1 * Rl + C3 * R1 * Rl,
      Rr + Rl
    ]
  )


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
plt.ylim([-40, 4])
plt.show()