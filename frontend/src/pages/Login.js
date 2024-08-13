import React from 'react';

function Login() {
  return (
<div class="min-h-screen flex items-center justify-center w-full bg-slate-200">
	<div class="bg-white shadow-md rounded-lg px-8 py-8 max-w-md">
		<form action="#">
			<div class="mb-4">
				<label for="email" class="block text-sm font-medium text-gray-700 mb-2">Email</label>
				<input type="email" id="email" class="shadow-sm rounded-md w-full px-3 py-2 border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500" placeholder="your@email.com" required />
			</div>
			<div class="mb-4">
				<label for="password" class="block text-sm font-medium text-gray-700 mb-2">Password</label>
				<input type="password" id="password" class="shadow-sm rounded-md w-full px-3 py-2 border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500" placeholder="Enter your password" required />
			</div>
			<button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">Login</button>
		</form>
	</div>
</div>
  );
}

export default Login;