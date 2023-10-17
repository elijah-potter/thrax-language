export default [
  {
    name: "tour.la",
    href: "/?code=println(%22Welcome%20to%20Thrax!%22)%3B%0A%0A%2F%2F%20This%20is%20a%20comment%0A%2F%2F%20Below%20is%20an%20%60if%60%20statement%20and%20a%20boolean%20expression.%0Aif%20(3%20%3D%3D%203)%7B%0A%20%20%20%20println(%22Thrax%20syntax%20should%20be%20pretty%20familiar%20to%20anyone%20who%20has%20used%20JavaScript%22)%3B%0A%7D%0A%0A%2F%2F%20%60while%60%20loops%20are%20the%20only%20option.%0Alet%20i%20%3D%200%3B%0Awhile(i%20%3C%205)%7B%0A%20%20%20%20println(%22This%20is%20a%20loop!%22)%3B%0A%20%20%20%20i%20%2B%3D%201%3B%0A%7D%0A%0Alet%20arr%20%3D%20%5B%22arrays%22%2C%20%22look%22%2C%20%22like%22%2C%20%22this%22%5D%3B%0A%0Ai%20%3D%200%3B%0Awhile(i%20%3C%20len(arr))%7B%0A%20%20%20%20println(arr%5Bi%5D)%3B%0A%20%20%20%20i%20%2B%3D%201%3B%0A%7D%0A%0Aprintln(%22We%20can%20define%20and%20use%20functions%20like%20this%3A%22)%3B%0A%0Afn%20greet(name)%7B%0A%20%20%20%20%2F%2F%20Strings%20can%20be%20concatonated%20with%20%60%2B%60%0A%20%20%20%20println(%22Hello%2C%20%22%20%2B%20name%20%2B%20%22!%22)%3B%0A%7D%0A%0Agreet(%22world%22)%3B",
  },
  {
    name: "add_fns.la",
    href: "/?code=fn%20test(n)%7B%0A%20%20return%20n%3B%0A%7D%0A%0Atest(1%20%2B%202)%20%2B%20test(2)%3B%0A",
  },
  {
    name: "assign_index.la",
    href: "/?code=let%20arr%20%3D%20%5B1%2C%202%2C%203%5D%3B%0A%0Aarr%3B%0Aarr%5B2%5D%3B%0A%0Aarr%5B2%5D%20%3D%2023%3B%0A%0Areturn%20arr%3B%0A",
  },
  {
    name: "break_continue.la",
    href: "/?code=let%20i%20%3D%2010%20**%203%3B%0A%0A%2F%2F%20Simulate%20a%20do-while%20loop%0Awhile%20(true)%7B%0A%20%20i%20-%3D%201%3B%20%0A%0A%20%20if%20(i%20%3D%3D%200)%7B%0A%20%20%20%20break%3B%0A%20%20%7D%0A%0A%20%20continue%3B%0A%0A%20%20while(true)%7B%0A%20%20%20%20%2F%2F%20It%20should%20never%20reach%20this%20point%2C%20it%20should%20have%20continued%20to%20the%20next%20iteration%20of%20loop.%0A%20%20%7D%0A%7D%0A%0Areturn%20i%3B%0A",
  },
  {
    name: "cyclic_arrays.la",
    href: "/?code=%2F%2F%20I%20honestly%20do%20not%20remember%20why%20I%20wrote%20this.%0A%2F%2F%20My%20best%20guess%20is%20that%20it%20is%20to%20test%20the%20GC%0Alet%20k%20%3D%200%3B%0A%0Awhile%20(k%20%3C%2010)%7B%0A%20%20let%20test_arr%20%3D%20%5B%5D%3B%0A%20%20let%20test_arr_2%20%3D%20%5B%5D%3B%0A%0A%20%20let%20i%20%3D%200%3B%0A%20%20while%20(i%20%3C%2010)%7B%0A%20%20%20%20push(test_arr%2C%20i)%3B%0A%20%20%20%20push(test_arr%2C%20test_arr_2)%3B%0A%20%20%20%20let%20temp%20%3D%20test_arr%3B%0A%20%20%20%20test_arr%20%3D%20test_arr_2%3B%0A%20%20%20%20test_arr_2%20%3D%20temp%3B%0A%20%20%20%20i%20%3D%20i%20%2B%201%3B%0A%20%20%7D%0A%0A%20%20k%20%3D%20k%20%2B%201%3B%0A%7D%0A",
  },
  { name: "empty_fn.la", href: "/?code=fn%20add(a%2C%20b)%7B%0A%7D%0A" },
  {
    name: "fib.la",
    href: "/?code=%2F%2F%20This%20is%20a%20really%20slow%20implementation%20of%20fib.%0A%2F%2F%20It%20can%20be%20used%20for%20testing%20the%20speed%20of%20the%20scripting%20engine%20by%20increased%20the%20value%20of%20%60up_to%60.%0A%2F%2F%20Inspired%20by%20a%20similar%20script%20in%20Rhai's%20examples%0A%0Afn%20fib(n)%20%7B%0A%20%20if%20(n%20%3C%202)%20%7B%0A%20%20%20%20return%20n%3B%0A%20%20%7D%20else%20%7B%0A%20%20%20%20return%20fib(n%20-%201)%20%2B%20fib(n%20-%202)%3B%0A%20%20%7D%0A%7D%0A%0Alet%20results%20%3D%20%5B%5D%3B%0A%0Alet%20i%20%3D%20%2F*%20this%20is%20a%20block%20comment%20*%2F%200%3B%0Alet%20up_to%20%3D%2018%3B%0A%0Awhile%20(i%20%3C%20up_to)%7B%0A%20%20let%20c%20%3D%20fib(i)%3B%0A%20%20push(results%2C%20c)%3B%0A%20%20i%20%2B%3D%201%3B%0A%7D%0A%0Areturn%20results%3B%0A",
  },
  {
    name: "index_object.la",
    href: "/?code=let%20k%20%3D%20%7B%0A%20%20t%3A%2023%2C%0A%20%20sdf%3A%20%22testing%22%2C%0A%20%20rec%3A%20%7B%0A%20%20%20%20a%3A%20%22inner%22%0A%20%20%7D%0A%7D%3B%0A%0Ak%5B%22t%22%5D%3B%0Ak%5B%22sdf%22%5D%3B%0Ak%5B%22rec%22%5D%3B%0A",
  },
  {
    name: "primes.la",
    href: "/?code=%2F%2F%20Implementation%20of%20the%20Seive%20of%20Eratosthenes%0A%0Afn%20array_filled_with(array_size%2C%20with)%7B%0A%20%20let%20arr%20%3D%20%5B%5D%3B%0A%0A%20%20while%20(len(arr)%20%3C%20array_size)%7B%0A%20%20%20%20push(arr%2C%20with)%3B%0A%20%20%7D%0A%0A%20%20return%20arr%3B%0A%7D%0A%0A%2F%2F%20Finds%20all%20primes%20from%202..%3Dn-1%0Afn%20primes_up_to(n)%7B%0A%20%20let%20i%20%3D%202%3B%20%20%0A%20%20let%20a%20%3D%20array_filled_with(n%2C%20true)%3B%0A%0A%20%20while%20(i%20%3C%20n)%7B%0A%20%20%20%20if%20(a%5Bi%5D)%7B%0A%20%20%20%20%20%20let%20j%20%3D%20i%20*%202%3B%0A%0A%20%20%20%20%20%20while%20(j%20%3C%20n)%7B%0A%20%20%20%20%20%20%20%20a%5Bj%5D%20%3D%20false%3B%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20j%20%2B%3D%20i%3B%0A%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%20%0A%0A%20%20%20%20i%20%2B%3D%201%3B%0A%20%20%7D%0A%0A%20%20let%20primes%20%3D%20%5B%5D%3B%20%20%0A%20%20let%20prime_n%20%3D%202%3B%0A%0A%20%20while(prime_n%20%3C%20len(a))%7B%0A%20%20%20%20if%20(a%5Bprime_n%5D)%7B%0A%20%20%20%20%20%20push(primes%2C%20prime_n)%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20prime_n%20%2B%3D%201%3B%0A%20%20%7D%0A%0A%20%20return%20primes%3B%0A%7D%0A%0Areturn%20primes_up_to(300)%3B%0A",
  },
  {
    name: "queue.la",
    href: "/?code=let%20queue%20%3D%20%5B8%2C%202%2C%2032%2C%2064%5D%3B%0A%0A%2F%2F%20Add%20items%20to%20the%20front%0Aunshift(queue%2C%202%20**%2016)%3B%0A%0Awhile%20(len(queue)%20%3E%200)%7B%0A%20%20%2F%2F%20You%20can%20take%20items%20off%20the%20front%20of%20the%20array%0A%20%20let%20item%20%3D%20shift(queue)%3B%0A%0A%20%20if%20(item%20%3E%201)%7B%0A%20%20%20%20%2F%2F%20..%20and%20push%20items%20onto%20the%20back%0A%20%20%20%20push(queue%2C%20item%20%2F%202)%3B%0A%20%20%7D%0A%7D%0A%0Areturn%20queue%3B%0A",
  },
  {
    name: "stack.la",
    href: "/?code=let%20stack%20%3D%20%5B1%2C%203%2C%202%2C%206%2C%2023%5D%3B%0A%0A%2F%2F%20We%20can%20push%20items%20to%20the%20end%20of%20the%20array%20%0Apush(stack%2C%20%22new%20item%22)%3B%0A%0A%2F%2F%20We%20can%20pop%20items%20off%20the%20end%0Apop(stack)%3B%0A%0Areturn%20pop(stack)%3B%0A",
  },
  {
    name: "timing.la",
    href: "/?code=%2F%2F%20Get%20the%20time%20(in%20milliseconds)%20since%20the%20Unix%20Epoch%0Alet%20time%20%3D%20timestamp()%3B%0A%0A%2F%2F%20Similar%20to%20Python%2C%20%60a%20**%20b%60%20raises%20a%20to%20the%20power%20of%20b%0Alet%20i%20%3D%2010%20**%205%3B%0A%0Awhile%20(i%20%3E%200)%7B%0A%20%20i%20-%3D%201%3B%0A%7D%0A%0Areturn%20timestamp()%20-%20time%3B%0A",
  },
  {
    name: "while_loop.la",
    href: "/?code=while%20(false)%7B%0A%20%20let%20a%20%3D%201%20%2B%201%3B%0A%7D%0A",
  },
];
