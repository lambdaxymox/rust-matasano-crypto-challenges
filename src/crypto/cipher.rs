trait BlockCipher {
    fn process_block<W: WriteBuffer>(&self, input: &[u8], output_buf: &mut W);
}

trait BlockPadding {
    fn pad_input(&mut self, input: &mut W);

    fn strip_padding(&mut self, input: &mut R);
}