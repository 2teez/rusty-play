"use strict"

function tail_fn(num) {
	let total = 1;
	function tail_func(num, total) {
   if (num == 1) {
		 return total;
	 }
      return tail_func(num - 1, num * total);
	}
  return tail_func(num, total);
}

console.log(tail_fn(170));
