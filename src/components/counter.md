---
use crate::prelude::*;
use rasto::prelude::set_interval;

pub struct Props{
	pub count:f32
}
let mut count = signal(0);

set_interval(||{
	count 
})

---
<div> Count value is {count()}</div>
<button 
	on:click={ set_count.update(|count|count += 1) }>
	Click Me!
</button>
<style>


</style>