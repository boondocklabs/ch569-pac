#[doc = "Register `R8_SPI0_INTER_EN` reader"]
pub struct R(crate::R<R8_SPI0_INTER_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_SPI0_INTER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_SPI0_INTER_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_SPI0_INTER_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_SPI0_INTER_EN` writer"]
pub struct W(crate::W<R8_SPI0_INTER_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_SPI0_INTER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R8_SPI0_INTER_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_SPI0_INTER_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SPI_IE_CNT_END` reader - enable interrupt for SPI total byte count end"]
pub type RB_SPI_IE_CNT_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_CNT_END` writer - enable interrupt for SPI total byte count end"]
pub type RB_SPI_IE_CNT_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IE_BYTE_END` reader - enable interrupt for SPI byte exchanged"]
pub type RB_SPI_IE_BYTE_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_BYTE_END` writer - enable interrupt for SPI byte exchanged"]
pub type RB_SPI_IE_BYTE_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IE_FIFO_HF` reader - enable interrupt for SPI FIFO half"]
pub type RB_SPI_IE_FIFO_HF_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_FIFO_HF` writer - enable interrupt for SPI FIFO half"]
pub type RB_SPI_IE_FIFO_HF_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IE_DMA_END` reader - enable interrupt for SPI DMA completion"]
pub type RB_SPI_IE_DMA_END_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_DMA_END` writer - enable interrupt for SPI DMA completion"]
pub type RB_SPI_IE_DMA_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IE_FIFO_OV` reader - enable interrupt for SPI FIFO overflow"]
pub type RB_SPI_IE_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_FIFO_OV` writer - enable interrupt for SPI FIFO overflow"]
pub type RB_SPI_IE_FIFO_OV_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
#[doc = "Field `RB_SPI_IE_FST_BYTE` reader - enable interrupt for SPI slave mode first byte received"]
pub type RB_SPI_IE_FST_BYTE_R = crate::BitReader<bool>;
#[doc = "Field `RB_SPI_IE_FST_BYTE` writer - enable interrupt for SPI slave mode first byte received"]
pub type RB_SPI_IE_FST_BYTE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, R8_SPI0_INTER_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable interrupt for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_ie_cnt_end(&self) -> RB_SPI_IE_CNT_END_R {
        RB_SPI_IE_CNT_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_ie_byte_end(&self) -> RB_SPI_IE_BYTE_END_R {
        RB_SPI_IE_BYTE_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_hf(&self) -> RB_SPI_IE_FIFO_HF_R {
        RB_SPI_IE_FIFO_HF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_ie_dma_end(&self) -> RB_SPI_IE_DMA_END_R {
        RB_SPI_IE_DMA_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_ov(&self) -> RB_SPI_IE_FIFO_OV_R {
        RB_SPI_IE_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_ie_fst_byte(&self) -> RB_SPI_IE_FST_BYTE_R {
        RB_SPI_IE_FST_BYTE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_ie_cnt_end(&mut self) -> RB_SPI_IE_CNT_END_W<0> {
        RB_SPI_IE_CNT_END_W::new(self)
    }
    #[doc = "Bit 1 - enable interrupt for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_ie_byte_end(&mut self) -> RB_SPI_IE_BYTE_END_W<1> {
        RB_SPI_IE_BYTE_END_W::new(self)
    }
    #[doc = "Bit 2 - enable interrupt for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_hf(&mut self) -> RB_SPI_IE_FIFO_HF_W<2> {
        RB_SPI_IE_FIFO_HF_W::new(self)
    }
    #[doc = "Bit 3 - enable interrupt for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_ie_dma_end(&mut self) -> RB_SPI_IE_DMA_END_W<3> {
        RB_SPI_IE_DMA_END_W::new(self)
    }
    #[doc = "Bit 4 - enable interrupt for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_ov(&mut self) -> RB_SPI_IE_FIFO_OV_W<4> {
        RB_SPI_IE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 7 - enable interrupt for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_ie_fst_byte(&mut self) -> RB_SPI_IE_FST_BYTE_W<7> {
        RB_SPI_IE_FST_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_spi0_inter_en](index.html) module"]
pub struct R8_SPI0_INTER_EN_SPEC;
impl crate::RegisterSpec for R8_SPI0_INTER_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_spi0_inter_en::R](R) reader structure"]
impl crate::Readable for R8_SPI0_INTER_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_spi0_inter_en::W](W) writer structure"]
impl crate::Writable for R8_SPI0_INTER_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_SPI0_INTER_EN to value 0"]
impl crate::Resettable for R8_SPI0_INTER_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
