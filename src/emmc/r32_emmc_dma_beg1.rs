#[doc = "Register `R32_EMMC_DMA_BEG1` reader"]
pub struct R(crate::R<R32_EMMC_DMA_BEG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_EMMC_DMA_BEG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_EMMC_DMA_BEG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_EMMC_DMA_BEG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_EMMC_DMA_BEG1` writer"]
pub struct W(crate::W<R32_EMMC_DMA_BEG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_EMMC_DMA_BEG1_SPEC>;
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
impl From<crate::W<R32_EMMC_DMA_BEG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_EMMC_DMA_BEG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_DMAAD1_MASK` reader - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RB_EMMC_DMAAD1_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB_EMMC_DMAAD1_MASK` writer - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
pub type RB_EMMC_DMAAD1_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R32_EMMC_DMA_BEG1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad1_mask(&self) -> RB_EMMC_DMAAD1_MASK_R {
        RB_EMMC_DMAAD1_MASK_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - start address of read-write data buffer,the lower 4 bits are fixed to 0"]
    #[inline(always)]
    pub fn rb_emmc_dmaad1_mask(&mut self) -> RB_EMMC_DMAAD1_MASK_W<0> {
        RB_EMMC_DMAAD1_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits DMA start address register when to operate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_emmc_dma_beg1](index.html) module"]
pub struct R32_EMMC_DMA_BEG1_SPEC;
impl crate::RegisterSpec for R32_EMMC_DMA_BEG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_emmc_dma_beg1::R](R) reader structure"]
impl crate::Readable for R32_EMMC_DMA_BEG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_emmc_dma_beg1::W](W) writer structure"]
impl crate::Writable for R32_EMMC_DMA_BEG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_EMMC_DMA_BEG1 to value 0"]
impl crate::Resettable for R32_EMMC_DMA_BEG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
