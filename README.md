vmdrs-py
---
VMD, aka Variational Mode Decomposition, is a signal processing tool that decompse the input signal into different band-limited IMFs.

This project exposes the [vmd-rs](https://github.com/jiafuei/vmd-rs) crate to Python.

Installation
---
Available on [PyPI](https://pypi.org/project/vmdrs-py)
```
pip install vmdrs-py
```

| Target                    | Blas Provider        |
|---------------------------|----------------------|
| aarch64-unknown-linux-gnu | OpenBLAS             |
| x86_64-unknown-linux-gnu  | Intel MKL            |
| x86_64-pc-windows-msvc    | Intel MKL            |
| x86_64-apple-darwin       | Accelerate Framework |
| aarch64-apple-darwin      | Accelerate Framework |


Examples
---
```
#%% Simple example: generate signal with 3 components + noise  
import numpy as np  
import matplotlib.pyplot as plt  
from vmdpy import VMD  

#. Time Domain 0 to T  
T = 1000  
fs = 1/T  
t = np.arange(1,T+1)/T  
freqs = 2*np.pi*(t-0.5-fs)/(fs)  

#. center frequencies of components  
f_1 = 2  
f_2 = 24  
f_3 = 288  

#. modes  
v_1 = (np.cos(2*np.pi*f_1*t))  
v_2 = 1/4*(np.cos(2*np.pi*f_2*t))  
v_3 = 1/16*(np.cos(2*np.pi*f_3*t))  

f = v_1 + v_2 + v_3 + 0.1*np.random.randn(v_1.size)  

#. some sample parameters for VMD  
alpha = 2000       # moderate bandwidth constraint  
tau = 0.            # noise-tolerance (no strict fidelity enforcement)  
K = 3              # 3 modes  
DC = 0             # no DC part imposed  
init = 1           # initialize omegas uniformly  
tol = 1e-7  


#. Run VMD 
u, u_hat, omega = VMD(f, alpha, tau, K, DC, init, tol)  

#. Visualize decomposed modes
plt.figure()
plt.subplot(2,1,1)
plt.plot(f)
plt.title('Original signal')
plt.xlabel('time (s)')
plt.subplot(2,1,2)
plt.plot(u.T)
plt.title('Decomposed modes')
plt.xlabel('time (s)')
plt.legend(['Mode %d'%m_i for m_i in range(u.shape[0])])
plt.tight_layout()

```


Compiling from source
---

### Requirements

1. Python >= 3.7
2. Rustc

### Instructions
Rust compiler is needed since this project is compiled using [Maturin](https://github.com/PyO3/maturin). 
```
pip install maturin
git clone https://github.com/jiafuei/vmdrs-py.git && cd vmdrs-py
maturin build --release
```

### Customizing BLAS providers
BLAS is used by [ndarray](https://github.com/rust-ndarray/ndarray), follow the instructions there
and edit [Cargo.toml](./Cargo.toml).
