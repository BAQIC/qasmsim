import cudaq
from cudaq import spin

from typing import List

# We begin by defining the spin Hamiltonian for the system that we are working
# with. This is achieved through the use of `cudaq.SpinOperator`'s, which allow
# for the convenient creation of complex Hamiltonians out of Pauli spin operators.
hamiltonian = 5.907 - 2.1433 * spin.x(0) * spin.x(1) - 2.1433 * spin.y(
    0) * spin.y(1) + .21829 * spin.z(0) - 6.125 * spin.z(1)


# Next, using the `cudaq.Kernel`, we define the variational quantum circuit
# that we'd like to use as an ansatz.
# Create a kernel that takes a list of floats as a function argument.
@cudaq.kernel
def kernel(angles: List[float]):
    # Allocate 2 qubits.
    qubits = cudaq.qvector(2)
    x(qubits[0])
    # Apply an `ry` gate that is parameterized by the first value
    # of our `angles`.
    ry(angles[0], qubits[1])
    x.ctrl(qubits[1], qubits[0])
    # Note: the kernel must not contain measurement instructions.


# The last thing we need is to pick an optimizer from the suite of `cudaq.optimizers`.
# We can optionally tune this optimizer through its initial parameters, iterations,
# optimization bounds, etc. before passing it to `cudaq.vqe`.
optimizer = cudaq.optimizers.COBYLA()
# optimizer.max_iterations = ...
# optimizer...

# Finally, we pass all of that into `cudaq.vqe`, and it will automatically run our
# optimization loop, returning a tuple of the minimized eigenvalue of our `spin_operator`
# and the list of optimal variational parameters.

# hamiltonian
#  (0.00029,0)        (0,0)        (0,0)        (0,0)
#        (0,0) (-0.43629,0)  (-4.2866,0)        (0,0)
#        (0,0)  (-4.2866,0)  (12.2503,0)        (0,0)
#        (0,0)        (0,0)        (0,0)  (11.8137,0)
# observe for angle = 0
# -0.4362899999999996
print(hamiltonian.to_matrix())
ob = cudaq.observe(kernel, hamiltonian, [0,])
print(ob.expectation())

energy, parameter = cudaq.vqe(
    kernel=kernel,
    spin_operator=hamiltonian,
    optimizer=optimizer,
    # list of parameters has length of 1:
    parameter_count=1)

print(f"\nminimized <H> = {round(energy,16)}")
print(f"optimal theta = {round(parameter[0],16)}")