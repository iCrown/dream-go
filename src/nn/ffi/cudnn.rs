// Copyright 2017 Karl Sundequist Blomdahl <karl.sundequist.blomdahl@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libc::{c_double, c_int, c_void, size_t};
use nn::ffi::cuda::{self, Stream};

#[repr(i32)]
#[allow(dead_code)]
pub enum ActivationMode {
    Relu = 1,
    Tanh = 2
}

#[repr(i32)]
#[allow(dead_code)]
pub enum BatchNormMode {
    PerActivation = 0,
    Spatial = 1
}

#[repr(i32)]
pub enum ConvolutionMode {
    CrossCorrelation = 1
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum ConvolutionFwdAlgo {
    ImplicitPrecompGemm = 1,
    Winograd = 6,
    WinogradNonFused = 7
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum ConvolutionFwdPreference {
    NoWorkspace = 0,
    PreferFastest = 1,
    SpecifyWorkspaceLimit = 2
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum DataType {
    Float = 0,
    Half = 2,
    Int8 = 3,
    Int32 = 4,
    Int8x4 = 5,
}

impl DataType {
    pub fn size(&self) -> usize {
        match *self {
            DataType::Float => 4,
            DataType::Half => 2,
            DataType::Int8 => 1,
            DataType::Int32 => 4,
            DataType::Int8x4 => 1
        }
    }

    pub fn to_cuda(&self) -> cuda::DataType {
        match *self {
            DataType::Float => cuda::DataType::R32F,
            DataType::Half => cuda::DataType::R16F,
            DataType::Int8 => cuda::DataType::R8I,
            DataType::Int32 => cuda::DataType::R32I,
            DataType::Int8x4 => panic!()
        }
    }
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OpTensorOp {
    Add = 0
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Status {
    Success = 0,
    NotInitialized = 1,
    AllocFailed = 2,
    BadParam = 3,
    InternalError = 4,
    InvalidValue = 5,
    ArchMismatch = 6,
    MappingError = 7,
    ExecutionFailed = 8,
    NotSupported = 9,
    LicenseError = 10,
    RuntimePrerequisiteMissing = 11
}

impl Status {
    /// Returns whether this status indicates a successful call.
    pub fn is_ok(&self) -> bool {
        *self == Status::Success
    }
}

#[repr(i32)]
#[cfg(feature = "tensor-core")]
pub enum MathType {
    TensorOpMath = 1
}

#[repr(i32)]
#[allow(dead_code)]
pub enum NanPropagation {
    NotPropagateNan = 0,
    PropagateNan = 1
}

#[repr(i32)]
#[allow(dead_code)]
pub enum SoftmaxAlgorithm {
    Fast = 0,
    Accurate = 1,
    Log = 2
}

#[repr(i32)]
#[allow(dead_code)]
pub enum SoftmaxMode {
    Instance = 0,
    Channel = 1
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum TensorFormat {
    NCHW = 0,
    NHWC = 1,
    NCHWVECTC = 2
}

pub type ActivationDescriptor = *const c_void;
pub type ConvolutionDescriptor = *const c_void;
pub type FilterDescriptor = *const c_void;
pub type Handle = *const c_void;
pub type OpTensorDescriptor = *const c_void;
pub type TensorDescriptor = *const c_void;

#[link(name = "cudnn")]
extern {
    pub fn cudnnCreate(handle: *mut Handle) -> Status;
    pub fn cudnnDestroy(handle: Handle) -> Status;

    /// This function copies the scaled data from one tensor to another tensor with a different
    /// layout. Those descriptors need to have the same dimensions but not necessarily the
    /// same strides. The input and output tensors must not overlap in any way (i.e., tensors
    /// cannot be transformed in place). This function can be used to convert a tensor with an
    /// unsupported format to a supported one.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `alpha` -
    /// * `xDesc` -
    /// * `x` -
    /// * `beta` -
    /// * `yDesc` -
    /// * `y` -
    /// 
    #[allow(dead_code)]
    pub fn cudnnTransformTensor(
        handle: Handle,
        alpha: *const f32,
        xDesc: TensorDescriptor,
        x: *const c_void,
        beta: *const f32,
        yDesc: TensorDescriptor,
        y: *mut c_void,
    ) -> Status;

    /// This function sets the cuDNN library stream, which will be used to execute all
    /// subsequent calls to the cuDNN library functions with that particular handle. If the
    /// cuDNN library stream is not set, all kernels use the default (NULL) stream. In particular,
    /// this routine can be used to change the stream between kernel launches and then to reset
    /// the cuDNN library stream back to `NULL`.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `stream` -
    ///
    pub fn cudnnSetStream(handle: Handle, stream: Stream) -> Status;

    pub fn cudnnCreateActivationDescriptor(activationDesc: *mut ActivationDescriptor) -> Status;
    pub fn cudnnDestroyActivationDescriptor(activationDesc: ActivationDescriptor) -> Status;

    /// This function initializes a previously created generic activation descriptor object
    /// 
    /// # Arguments
    /// 
    /// * `activationDesc` - Handle to a previously created pooling descriptor.
    /// * `mode` - Enumerant to specify the activation mode.
    /// * `reluNanOpt` - Enumerant to specify the Nan propagation mode.
    /// * `coef` - floating point number to specify the clipping threashold when the activation
    ///   mode is set to CUDNN_ACTIVATION_CLIPPED_RELU or to specify the alpha
    ///   coefficient when the activation mode is set to CUDNN_ACTIVATION_ELU.
    ///
    pub fn cudnnSetActivationDescriptor(
        activationDesc: ActivationDescriptor,
        mode: ActivationMode,
        reluNanOpt: NanPropagation,
        coef: c_double
    ) -> Status;

    pub fn cudnnCreateTensorDescriptor(tensorDesc: *mut TensorDescriptor) -> Status;
    pub fn cudnnDestroyTensorDescriptor(tensorDesc: TensorDescriptor) -> Status;

    /// This function initializes a previously created generic Tensor descriptor object into a
    /// 4D tensor. The strides of the four dimensions are inferred from the format parameter
    /// and set in such a way that the data is contiguous in memory with no padding between
    /// dimensions.
    /// 
    /// # Arguments
    /// 
    /// * `tensorDesc` - Handle to a previously created tensor descriptor.
    /// * `format` - Type of format.
    /// * `dataType` - Data type.
    /// * `n` - Number of images.
    /// * `c` - Number of feature maps per image.
    /// * `h` - Height of each feature map.
    /// * `w` - Width of each feature map.
    ///
    pub fn cudnnSetTensor4dDescriptor(
        tensorDesc: TensorDescriptor,
        format: TensorFormat,
        dataType: DataType,
        n: c_int,  // batch
        c: c_int,  // channels
        h: c_int,  // height
        w: c_int,  // width
    ) -> Status;

    pub fn cudnnCreateFilterDescriptor(filterDesc: *mut FilterDescriptor) -> Status;
    pub fn cudnnDestroyFilterDescriptor(filterDesc: FilterDescriptor) -> Status;

    /// This function initializes a previously created filter descriptor object into a 4D filter.
    /// Filters layout must be contiguous in memory.
    /// 
    /// # Arguments
    /// 
    /// * `filterDesc` - Handle to a previously created filter descriptor.
    /// * `dataType` - Data type.
    /// * `format` - Type of format.
    /// * `k` - Number of output feature maps.
    /// * `c` - Number of input feature maps.
    /// * `h` - Height of each filter.
    /// * `w` - Width of each filter.
    ///
    pub fn cudnnSetFilter4dDescriptor(
        filterDesc: FilterDescriptor,
        dataType: DataType,
        format: TensorFormat,
        k: c_int,  // output features
        c: c_int,  // input features
        h: c_int,  // height
        w: c_int,  // width
    ) -> Status;

    pub fn cudnnCreateConvolutionDescriptor(convDesc: *mut ConvolutionDescriptor) -> Status;
    pub fn cudnnDestroyConvolutionDescriptor(convDesc: ConvolutionDescriptor) -> Status;

    /// This function initializes a previously created convolution descriptor object into a 2D
    /// correlation. This function assumes that the tensor and filter descriptors corresponds
    /// to the formard convolution path and checks if their settings are valid. That same
    /// convolution descriptor can be reused in the backward path provided it corresponds to
    /// the same layer.
    /// 
    /// # Arguments
    /// 
    /// * `convDesc` - Handle to a previously created convolution descriptor.
    /// * `pad_h` - zero-padding height: number of rows of zeros implicitly concatenated
    ///   onto the top and onto the bottom of input images.
    /// * `pad_w` - zero-padding width: number of columns of zeros implicitly concatenated
    ///   onto the left and onto the right of input images.
    /// * `u` - Vertical filter stride.
    /// * `v` - Horizontal filter stride.
    /// * `dilation_h` - Filter height dilation.
    /// * `dilation_w` - Filter width dilation.
    /// * `mode` - Selects between `CUDNN_CONVOLUTION` and `CUDNN_CROSS_CORRELATION`.
    /// * `computeType` - compute precision
    ///
    pub fn cudnnSetConvolution2dDescriptor(
        convDesc: ConvolutionDescriptor,
        pad_h: c_int,
        pad_w: c_int,
        u: c_int,
        v: c_int,
        dilation_h: c_int,
        dilation_w: c_int,
        mode: ConvolutionMode,
        computeType: DataType
    ) -> Status;

    /// This function allows the user to specify whether or not the use of tensor op is permitted
    /// in library routines associated with a given convolution descriptor.
    /// 
    /// # Arguments
    /// 
    /// * `convDesc` -
    /// * `mathType` -
    /// 
    #[cfg(feature = "tensor-core")]
    pub fn cudnnSetConvolutionMathType(
        convDesc: ConvolutionDescriptor,
        mathType: MathType
    ) -> Status;

    /// This function adds the scaled values of a bias tensor to another tensor. Each dimension
    /// of the bias tensor A must match the corresponding dimension of the destination tensor
    /// C or must be equal to 1. In the latter case, the same value from the bias tensor for those
    /// dimensions will be used to blend into the C tensor.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `alpha` - 
    /// * `aDesc` - Handle to a previously initialized tensor descriptor.
    /// * `A` - Pointer to data of the tensor described by the `aDesc` descriptor.
    /// * `beta` - 
    /// * `cDesc` - Handle to a previously initialized tensor descriptor.
    /// * `C` - Pointer to data of the tensor described by the `cDesc` descriptor.
    ///
    pub fn cudnnAddTensor(
        handle: Handle,
        alpha: *const f32,
        aDesc: TensorDescriptor,
        A: *const c_void,
        beta: *const f32,
        cDesc: TensorDescriptor,
        C: *mut c_void
    ) -> Status;

    /// This function scale all the elements of a tensor by a given factor.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `yDesc` - Handle to a previously initialized tensor descriptor.
    /// * `y` - Pointer to data of the tensor described by the `yDesc` descriptor.
    /// * `alpha` - 
    /// 
    pub fn cudnnScaleTensor(
        handle: Handle,
        yDesc: TensorDescriptor,
        Y: *mut c_void,
        alpha: *const f32
    ) -> Status;

    /// This routine applies a specified neuron activation function element-wise over each input
    /// value.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `activationDesc` - Activation descriptor.
    /// * `alpha` - 
    /// * `srcDesc` - Handle to the previously initialized input tensor descriptor.
    /// * `src` - Data pointer to GPU memory associated with the tensor descriptor `srcDesc`.
    /// * `beta` - 
    /// * `destDesc` - Handle to the previously initialized output tensor descriptor.
    /// * `dest` - Data pointer to GPU memory associated with the tensor descriptor `destDesc`.
    ///
    pub fn cudnnActivationForward(
        handle: Handle,
        activationDesc: ActivationDescriptor,
        alpha: *const f32,
        srcDesc: TensorDescriptor,
        src: *const c_void,
        beta: *const f32,
        destDesc: TensorDescriptor,
        dest: *mut c_void,
    ) -> Status;

    /// This function serves as a heuristic for obtaining the best suited algorithm for
    /// `cudnnConvolutionForward` for the given layer specifications. Based on the input
    /// preference, this function will either return the fastest algorithm or the fastest algorithm
    /// within a given memory limit. For an exhaustive search for the fastest algorithm, please
    /// use `cudnnFindConvolutionForwardAlgorithm`.
    /// 
    /// # Arguments
    /// 
    /// * `handle` -
    /// * `xDesc` -
    /// * `wDesc` -
    /// * `convDesc` -
    /// * `yDesc` -
    /// * `preference` -
    /// * `memorySizeInBytes` -
    /// * `algo` -
    /// 
    pub fn cudnnGetConvolutionForwardAlgorithm(
        handle: Handle,
        xDesc: TensorDescriptor,
        wDesc: FilterDescriptor,
        convDesc: ConvolutionDescriptor,
        yDesc: TensorDescriptor,
        preference: ConvolutionFwdPreference,
        memorySizeInBytes: size_t,
        algo: *mut ConvolutionFwdAlgo
    ) -> Status;

    /// This function returns the amount of GPU memory workspace the user needs
    /// to allocate to be able to call cudnnConvolutionForward with the specified
    /// algorithm. The workspace allocated will then be passed to the routine
    /// `cudnnConvolutionForward`. The specified algorithm can be the result of the call to
    /// `cudnnGetConvolutionForwardAlgorithm` or can be chosen arbitrarily by the user.
    /// Note that not every algorithm is available for every configuration of the input tensor
    /// and/or every configuration of the convolution descriptor.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `xDesc` - Handle to the previously initialized x tensor descriptor.
    /// * `wDesc` - Handle to a previously initialized filter descriptor.
    /// * `convDesc` - Previously initialized convolution descriptor.
    /// * `yDesc` - Handle to the previously initialized y tensor descriptor.
    /// * `algo` - Enumerant that specifies the chosen convolution algorithm.
    /// * `sizeInBytes` - Amount of GPU memory needed as workspace to be able to execute a forward
    ///    convolution with the specified `algo`.
    ///
    pub fn cudnnGetConvolutionForwardWorkspaceSize(
        handle: Handle,
        xDesc: TensorDescriptor,
        wDesc: FilterDescriptor,
        convDesc: ConvolutionDescriptor,
        yDesc: TensorDescriptor,
        algo: ConvolutionFwdAlgo,
        sizeInBytes: *mut size_t
    ) -> Status;

    /// This function executes convolutions or cross-correlations over x using filters specified
    /// with w, returning results in y. Scaling factors alpha and beta can be used to scale the
    /// input tensor and the output tensor respectively.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `alpha` - 
    /// * `xDesc` - Handle to a previously initialized tensor descriptor.
    /// * `x` - Data pointer to GPU memory associated with the tensor descriptor `xDesc`.
    /// * `wDesc` - Handle to a previously initialized filter descriptor.
    /// * `w` - Data pointer to GPU memory associated with the filter descriptor wDesc.
    /// * `convDesc` - Previously initialized convolution descriptor.
    /// * `algo` - Enumerant that specifies which convolution algorithm shoud be used to
    ///   compute the results
    /// * `workSpace` - Data pointer to GPU memory to a workspace needed to able to execute
    ///   the specified algorithm. If no workspace is needed for a particular
    ///   algorithm, that pointer can be nil.
    /// * `workSpaceSizeInBytes` - Specifies the size in bytes of the provided workSpace
    /// * `beta` - 
    /// * `yDesc` - Handle to a previously initialized tensor descriptor.
    /// * `y` - Data pointer to GPU memory associated with the tensor descriptor yDesc
    ///   that carries the result of the convolution.
    ///
    pub fn cudnnConvolutionForward(
        handle: Handle,
        alpha: *const f32,
        xDesc: TensorDescriptor,
        x: *const c_void,
        wDesc: FilterDescriptor,
        w: *const c_void,
        convDesc: ConvolutionDescriptor,
        algo: ConvolutionFwdAlgo,
        workSpace: *mut c_void,
        workSpaceSizeInBytes: size_t,
        beta: *const f32,
        yDesc: TensorDescriptor,
        y: *mut c_void
    ) -> Status;

    /// This function applies a bias and then an activation to the convolutions or
    /// crosscorrelations of `cudnnConvolutionForward()`, returning results in
    /// `y`. The full computation follows the equation
    /// 
    /// `y = act ( alpha1 * convDesc(x) + alpha2 * z + bias )`
    /// 
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `alpha1` - Pointers to scaling factors (in host memory).
    /// * `xDesc` - Handle to a previously initialized tensor descriptor.
    /// * `x` - Data pointer to GPU memory associated with the tensor descriptor `xDesc`.
    /// * `wDesc` - Handle to a previously initialized filter descriptor.
    /// * `w` - Data pointer to GPU memory associated with the filter descriptor `wDesc`.
    /// * `convDesc` - Previously initialized convolution descriptor.
    /// * `algo` - Enumerant that specifies which convolution algorithm shoud be used to
    ///   compute the results
    /// * `workSpace` - Data pointer to GPU memory to a workspace needed to able to execute the
    ///   specified algorithm.
    /// * `workSpaceSizeInBytes` -  Specifies the size in bytes of the provided `workSpace`.
    /// * `alpha2` - Pointers to scaling factors (in host memory).
    /// * `zDesc` - Handle to a previously initialized tensor descriptor.
    /// * `z` - Data pointer to GPU memory associated with the tensor descriptor `zDesc`.
    /// * `biasDesc` - Handle to a previously initialized tensor descriptor.
    /// * `bias` -  Data pointer to GPU memory associated with the tensor descriptor `biasDesc`.
    /// * `activationDesc` - Handle to a previously initialized activation descriptor.
    /// * `yDesc` - Handle to a previously initialized tensor descriptor.
    /// * `y` - Data pointer to GPU memory associated with the tensor descriptor
    ///   `yDesc` that carries the result of the convolution.
    /// 
    pub fn cudnnConvolutionBiasActivationForward(
        handle: Handle,
        alpha1: *const f32,
        xDesc: TensorDescriptor,
        x: *const c_void,
        wDesc: FilterDescriptor,
        w: *const c_void,
        convDesc: ConvolutionDescriptor,
        algo: ConvolutionFwdAlgo,
        workSpace: *mut c_void,
        workSpaceSizeInBytes: size_t,
        alpha2: *const f32,
        zDesc: TensorDescriptor,
        z: *const c_void,
        biasDesc: TensorDescriptor,
        bias: *const c_void,
        activationDesc: ActivationDescriptor,
        yDesc: TensorDescriptor,
        y: *mut c_void
    ) -> Status;

    /// This routine computes the softmax function.
    /// 
    /// # Arguments
    /// 
    /// * `handle` - Handle to a previously created cuDNN context.
    /// * `algorithm` - Enumerant to specify the softmax algorithm.
    /// * `mode` - Enumerant to specify the softmax mode.
    /// * `alpha` - 
    /// * `xDesc` - Handle to the previously initialized input tensor descriptor
    /// * `x` - Data pointer to GPU memory associated with the tensor descriptor xDesc
    /// * `beta` - 
    /// * `yDesc` - Handle to the previously initialized output tensor descriptor
    /// * `y` - Data pointer to GPU memory associated with the output tensor descriptor
    ///   yDesc.
    ///
    pub fn cudnnSoftmaxForward(
        handle: Handle,
        algorithm: SoftmaxAlgorithm,
        mode: SoftmaxMode,
        alpha: *const f32,
        xDesc: TensorDescriptor,
        x: *const c_void,
        beta: *const f32,
        yDesc: TensorDescriptor,
        y: *mut c_void
    ) -> Status;
}
