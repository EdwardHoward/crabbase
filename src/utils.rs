pub fn generate_id(size: usize) -> String {
  use rand::Rng;
  const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                          0123456789";
  let mut rng = rand::thread_rng();

  let random_string: String = (0..size)
      .map(|_| {
          let idx = rng.gen_range(0..CHARSET.len());
          CHARSET[idx] as char
      })
      .collect();

  println!("{:?}", random_string);

  return random_string
}