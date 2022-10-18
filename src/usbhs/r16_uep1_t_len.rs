#[doc = "Register `R16_UEP1_T_LEN` reader"]
pub struct R(crate::R<R16_UEP1_T_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_UEP1_T_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_UEP1_T_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_UEP1_T_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R16_UEP1_T_LEN` writer"]
pub struct W(crate::W<R16_UEP1_T_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R16_UEP1_T_LEN_SPEC>;
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
impl From<crate::W<R16_UEP1_T_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R16_UEP1_T_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP1_T_LEN` reader - endpoint 1 transmittal length"]
pub type UEP1_T_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP1_T_LEN` writer - endpoint 1 transmittal length"]
pub type UEP1_T_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, R16_UEP1_T_LEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub fn uep1_t_len(&self) -> UEP1_T_LEN_R {
        UEP1_T_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub fn uep1_t_len(&mut self) -> UEP1_T_LEN_W<0> {
        UEP1_T_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 1 transmittal length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_uep1_t_len](index.html) module"]
pub struct R16_UEP1_T_LEN_SPEC;
impl crate::RegisterSpec for R16_UEP1_T_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_uep1_t_len::R](R) reader structure"]
impl crate::Readable for R16_UEP1_T_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r16_uep1_t_len::W](W) writer structure"]
impl crate::Writable for R16_UEP1_T_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R16_UEP1_T_LEN to value 0"]
impl crate::Resettable for R16_UEP1_T_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
