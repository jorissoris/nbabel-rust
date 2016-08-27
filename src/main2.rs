use std::io;
use std::io::Read;
use std::iter;
use std::thread;

static dt: f64 = 1e-3;

struct Star {
	m: f64,
	r: Vec<f64>,
	v: Vec<f64>,
	a: Vec<f64>,
	a0: Vec<f64>,
}

fn acceleration(s: &mut Vec<Star>){
	let mut rij: Vec<f64> = vec![0.0; 3];
	for si in 0..s.len() {
		s[si].a = vec![0.0; 3];
	}
	for si in 0..s.len() {
		for sj in (si + 1)..s.len() {
			for i in 0..3 {
				rij[i] = s[si].r[i] - s[sj].r[i];
			}

			let RdotR: f64 = (rij[0]*rij[0] + rij[1]*rij[1] + rij[2]*rij[2]).sqrt();
			let apre: f64 = 1.0/(RdotR.powi(3));
			for i in 1..3 {
				s[si].a[i] -= s[sj].m*apre*rij[i];
				s[sj].a[i] += s[si].m*apre*rij[i];
			}
		}
	}
}

fn updatePositions(s: &mut Vec<Star>) {
	for star in s {
		for i in 1..3 {
			star.a0[i] = star.a[i];
			star.r[i] += dt*star.v[i] + 0.5*dt*dt*star.a0[i];
		}
	}
}

fn updateVelocities(s: &mut Vec<Star>) {
	for star in s {
		for i in 1..3 {
			star.v[i] += 0.5*dt*(star.a0[i] + star.a[i]);
			star.a0[i] = star.a[i];
		}
	}
}

fn energies(tos: &Vec<Star>) -> Vec<f64> {
	let ref s = *tos;
	let mut E: Vec<f64> = vec![0.0; 3];
	let mut rij: f64 = 0.0;

	//Kinetic energy
	for star in s {
		E[1] += 0.5*star.m*((star.v[0].powi(2) + star.v[1].powi(2) + star.v[2].powi(2)).sqrt());
	}

	for si in 0..s.len() {
		for sj in (si + 1)..s.len() {
			rij = 0.0;
			for i in 0..3 {
				rij += (s[si].r[i] - s[sj].r[i]).powi(2);
			}
			E[2] -= s[si].m*s[sj].m/(rij.sqrt());
		}
	}
	E[0] = E[1] + E[2];
	return E;
}

fn main() {
	let mut s: Vec<Star> = vec![];
	let mut line_buffer = String::new();
	let mut t: f64 = 0.0;
	let tend: f64 = 0.1;
	let mut k = 0;

	io::stdin().read_to_string(&mut line_buffer).expect("Something went wrong");

	let lines = line_buffer.split("\n");

	for line in lines {
		let mut r: Vec<f64> = Vec::with_capacity(3);
		let mut v: Vec<f64> = Vec::with_capacity(3);
		let mut m: f64;
		let mut dummy: f64; // Dummy is actually an u32, but who cares and this is shorter
		if line == "" {
			continue;
		}
		let mut var = line.split(" ");
		let mut arr: Vec<f64> = Vec::with_capacity(8);
		for num in var {
			if num == "" {
				continue;
			}
			arr.push(num.parse().expect("Invalid input"));
		}
		dummy = arr[0];
		m = arr[1];
		for i in 2..5 {
			r.push(arr[i]);
		}
		for i in 5..8 {
			v.push(arr[i]);
		}
		s.push(Star { m: m, r: r, v: v, a: vec![0.0; 3], a0: vec![0.0; 3] });
	}

	let mut E: Vec<f64>;
	let E0: Vec<f64> = energies(&s);
	println!("Energies: {} {} {}", E0[0], E0[1], E0[2]);

	acceleration(&mut s);

	while t < tend {
		updatePositions(&mut s);
		acceleration(&mut s);
		updateVelocities(&mut s);

		t += dt;
		k += 1; //Ugh, Rust doesn't support k++;

		if k % 10 == 0 {
			E = energies(&s);
			println!("t = {}, E = {} {} {}, dE = {}", t, E[0], E[1], E[2], (E[0]-E0[0])/E0[0]);
		}
	}
}
