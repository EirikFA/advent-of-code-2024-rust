use super::Solver;

#[derive(Debug, Clone)]
pub struct DataBlock(usize, usize, usize);

#[derive(Debug, Clone)]
pub struct FreeBlock(usize, usize);

pub struct Day9 {}

impl Solver for Day9 {
  type Input = (Vec<DataBlock>, Vec<FreeBlock>);

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    let mut data_blocks: Vec<DataBlock> = Vec::new();
    let mut free_blocks: Vec<FreeBlock> = Vec::new();

    let mut block_index = 0;
    for (i, char) in input.chars().enumerate() {
      let size: usize = char.to_string().parse().unwrap();
      if i % 2 == 0 {
        data_blocks.push(DataBlock(i / 2, block_index, size));
      } else {
        free_blocks.push(FreeBlock(block_index, size));
      }
      block_index += size;
    }

    (data_blocks, free_blocks)
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(1928.to_string())
  }

  fn part_1((data, free): &Self::Input) -> Self::Output1 {
    let mut data = data.clone();
    let mut free = free.clone();

    let mut data_i = data.len() - 1;
    let mut free_i = 0;

    while free_i < free.len() {
      let FreeBlock(free_pos, free_size) = free[free_i];
      if free_size == 0 {
        free_i += 1;
        continue;
      }
      let DataBlock(data_id, data_pos, data_size) = data[data_i];

      if free_pos >= data_pos {
        break;
      }

      if free_size >= data_size {
        data[data_i].1 = free_pos;
        data_i -= 1;
        free[free_i] = FreeBlock(free_pos + data_size, free_size - data_size);
      } else {
        data[data_i].2 = data_size - free_size;
        data.push(DataBlock(data_id, free_pos, free_size));
        free_i += 1;
      }
    }

    Self::file_system_checksum(&data).to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(2858.to_string())
  }

  fn part_2((data, free): &Self::Input) -> Self::Output2 {
    let mut data = data.clone();
    let mut free = free.clone();

    for data_i in (0..data.len()).rev() {
      let DataBlock(_data_id, data_pos, data_size) = data[data_i];
      if let Some(free_i) = free
        .iter()
        .position(|FreeBlock(free_pos, free_size)| *free_pos < data_pos && *free_size >= data_size)
      {
        let FreeBlock(free_pos, free_size) = free[free_i];
        data[data_i].1 = free_pos;
        free[free_i] = FreeBlock(free_pos + data_size, free_size - data_size);
      }
    }

    Self::file_system_checksum(&data).to_string()
  }
}

impl Day9 {
  fn file_system_checksum(data: &Vec<DataBlock>) -> usize {
    // data
    //   .iter()
    //   .fold(0, |checksum, DataBlock(id, pos, size)| {
    //     let pos_sum = (size * (2 * pos + size - 1)) as f64 / 2.0;
    //     checksum + (*id as f64 * pos_sum) as usize
    //   })
    //   .to_string()

    // Probably more efficient than the above (no floats and less multiplications)
    data.iter().fold(0, |checksum, DataBlock(id, pos, size)| {
      (0..*size).fold(checksum, |acc, i| acc + id * (pos + i))
    })
  }
}
