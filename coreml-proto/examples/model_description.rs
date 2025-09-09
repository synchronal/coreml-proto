use coreml_proto::proto::Model;
use prost::Message;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let model_path = "./fixtures/model.mlmodel";

  println!("Reading CoreML model from: {}", model_path);
  println!("{}", "=".repeat(51));

  let model_bytes = fs::read(model_path)?;
  let model = Model::decode(&model_bytes[..])?;

  println!("\nModel Information:");
  println!("  Specification Version: {}", model.specification_version);
  println!("  Is Updatable: {}", model.is_updatable);

  if let Some(ref description) = model.description {
    println!("\n  Model Description:");

    if !description.input.is_empty() {
      println!("\n  Inputs ({} total):", description.input.len());
      for (i, feature) in description.input.iter().enumerate() {
        println!("    {}. Name: {}", i + 1, feature.name);
        println!("       Description: {}", feature.short_description);
        if let Some(ref feature_type) = feature.r#type {
          print_feature_type(&feature_type, "       ");
        }
      }
    }

    if !description.output.is_empty() {
      println!("\n  Outputs ({} total):", description.output.len());
      for (i, feature) in description.output.iter().enumerate() {
        println!("    {}. Name: {}", i + 1, feature.name);
        println!("       Description: {}", feature.short_description);
        if let Some(ref feature_type) = feature.r#type {
          print_feature_type(&feature_type, "       ");
        }
      }
    }

    if !description.predicted_feature_name.is_empty() {
      println!("\n  🎯 Predicted Feature: {}", description.predicted_feature_name);
    }

    if !description.predicted_probabilities_name.is_empty() {
      println!("  📊 Predicted Probabilities: {}", description.predicted_probabilities_name);
    }

    if let Some(ref metadata) = description.metadata {
      println!("\n  Metadata:");
      if !metadata.short_description.is_empty() {
        println!("    Short Description: {}", metadata.short_description);
      }
      if !metadata.version_string.is_empty() {
        println!("    Version: {}", metadata.version_string);
      }
      if !metadata.author.is_empty() {
        println!("    Author: {}", metadata.author);
      }
      if !metadata.license.is_empty() {
        println!("    License: {}", metadata.license);
      }
      if !metadata.user_defined.is_empty() {
        println!("    User Defined Properties:");
        for (key, value) in &metadata.user_defined {
          println!("      {}: {}", key, value);
        }
      }
    }

    if !description.training_input.is_empty() {
      println!("\n  🏋️  Training Inputs ({} total):", description.training_input.len());
      for (i, feature) in description.training_input.iter().enumerate() {
        println!("    {}. Name: {}", i + 1, feature.name);
        println!("       Description: {}", feature.short_description);
      }
    }
  }

  println!("\n🔧 Model Type:");
  if let Some(ref model_type) = model.r#type {
    use coreml_proto::proto::model::Type;
    match model_type {
      Type::PipelineClassifier(_) => println!("  Pipeline Classifier"),
      Type::PipelineRegressor(_) => println!("  Pipeline Regressor"),
      Type::Pipeline(pipeline) => {
        println!("  Pipeline");
        if !pipeline.models.is_empty() {
          println!("    Contains {} sub-models", pipeline.models.len());
        }
      }
      Type::GlmRegressor(_) => println!("  GLM Regressor"),
      Type::SupportVectorRegressor(_) => println!("  Support Vector Regressor"),
      Type::TreeEnsembleRegressor(_) => println!("  Tree Ensemble Regressor"),
      Type::NeuralNetworkRegressor(nn_regressor) => {
        println!("  Neural Network Regressor");
        print_neural_network_details(&nn_regressor.layers);
      }
      Type::BayesianProbitRegressor(_) => println!("  Bayesian Probit Regressor"),
      Type::GlmClassifier(_) => println!("  GLM Classifier"),
      Type::SupportVectorClassifier(_) => println!("  Support Vector Classifier"),
      Type::TreeEnsembleClassifier(_) => println!("  Tree Ensemble Classifier"),
      Type::NeuralNetworkClassifier(nn_classifier) => {
        println!("  Neural Network Classifier");
        print_neural_network_details(&nn_classifier.layers);

        if let Some(ref class_labels) = nn_classifier.class_labels {
          use coreml_proto::proto::neural_network_classifier::ClassLabels;
          match class_labels {
            ClassLabels::StringClassLabels(labels) => {
              println!("    Class Labels (String): {:?}", labels.vector);
            }
            ClassLabels::Int64ClassLabels(labels) => {
              println!("    Class Labels (Int64): {:?}", labels.vector);
            }
          }
        }
      }
      Type::KNearestNeighborsClassifier(_) => println!("  K-Nearest Neighbors Classifier"),
      Type::NeuralNetwork(nn) => {
        println!("  Neural Network");
        print_neural_network_details(&nn.layers);
      }
      Type::ItemSimilarityRecommender(_) => println!("  Item Similarity Recommender"),
      Type::MlProgram(program) => {
        println!("  ML Program");
        println!("    Version: {}", program.version);
        if !program.functions.is_empty() {
          println!("    Functions: {}", program.functions.len());
          for (name, _) in &program.functions {
            println!("      - {}", name);
          }
        }
      }
      Type::CustomModel(custom) => {
        println!("  Custom Model");
        println!("    Class Name: {}", custom.class_name);
        if !custom.parameters.is_empty() {
          println!("    Parameters: {} defined", custom.parameters.len());
        }
      }
      Type::LinkedModel(linked) => {
        println!("  Linked Model");
        if let Some(ref link_type) = linked.link_type {
          use coreml_proto::proto::linked_model::LinkType;
          match link_type {
            LinkType::LinkedModelFile(file) => {
              if let Some(ref file_name) = file.linked_model_file_name {
                println!("    Linked File: {}", file_name.default_value);
              }
              if let Some(ref search_path) = file.linked_model_search_path {
                println!("    Search Path: {}", search_path.default_value);
              }
            }
          }
        }
      }
      Type::ClassConfidenceThresholding(_) => println!("  Class Confidence Thresholding"),
      Type::OneHotEncoder(_) => println!("  One Hot Encoder"),
      Type::Imputer(_) => println!("  Imputer"),
      Type::FeatureVectorizer(_) => println!("  Feature Vectorizer"),
      Type::DictVectorizer(_) => println!("  Dictionary Vectorizer"),
      Type::Scaler(_) => println!("  Scaler"),
      Type::CategoricalMapping(_) => println!("  Categorical Mapping"),
      Type::Normalizer(_) => println!("  Normalizer"),
      Type::ArrayFeatureExtractor(_) => println!("  Array Feature Extractor"),
      Type::NonMaximumSuppression(_) => println!("  Non-Maximum Suppression"),
      Type::Identity(_) => println!("  Identity"),
      Type::TextClassifier(_) => println!("  Text Classifier"),
      Type::WordTagger(_) => println!("  Word Tagger"),
      Type::Gazetteer(_) => println!("  Gazetteer"),
      Type::WordEmbedding(_) => println!("  Word Embedding"),
      Type::VisionFeaturePrint(_) => println!("  Vision Feature Print"),
      Type::SoundAnalysisPreprocessing(_) => println!("  Sound Analysis Preprocessing"),
      Type::AudioFeaturePrint(_) => println!("  Audio Feature Print"),
      Type::SerializedModel(_) => println!("  Serialized Model"),
    }
  }

  println!();
  println!("Model successfully decoded and analyzed!");

  Ok(())
}

fn print_feature_type(feature_type: &coreml_proto::proto::FeatureType, indent: &str) {
  use coreml_proto::proto::feature_type::Type;

  if let Some(ref type_variant) = feature_type.r#type {
    match type_variant {
      Type::Int64Type(_) => println!("{}Type: Int64", indent),
      Type::DoubleType(_) => println!("{}Type: Double", indent),
      Type::StringType(_) => println!("{}Type: String", indent),
      Type::ImageType(img) => {
        println!("{}Type: Image", indent);
        println!("{}  Width: {}", indent, img.width);
        println!("{}  Height: {}", indent, img.height);
        use coreml_proto::proto::image_feature_type::ColorSpace;
        match ColorSpace::try_from(img.color_space) {
          Ok(ColorSpace::InvalidColorSpace) | Err(_) => {}
          Ok(ColorSpace::Grayscale) => println!("{}  Color Space: Grayscale", indent),
          Ok(ColorSpace::Rgb) => println!("{}  Color Space: RGB", indent),
          Ok(ColorSpace::Bgr) => println!("{}  Color Space: BGR", indent),
          Ok(ColorSpace::GrayscaleFloat16) => println!("{}  Color Space: Grayscale Float16", indent),
        }
      }
      Type::MultiArrayType(arr) => {
        println!("{}Type: MultiArray", indent);
        if !arr.shape.is_empty() {
          println!("{}  Shape: {:?}", indent, arr.shape);
        }
        use coreml_proto::proto::array_feature_type::ArrayDataType;
        match ArrayDataType::try_from(arr.data_type) {
          Ok(ArrayDataType::Double) => println!("{}  Data Type: Double", indent),
          Ok(ArrayDataType::Float16) => println!("{}  Data Type: Float16", indent),
          Ok(ArrayDataType::Float32) => println!("{}  Data Type: Float32", indent),
          Ok(ArrayDataType::Int32) => println!("{}  Data Type: Int32", indent),
          Ok(ArrayDataType::Int8) => println!("{}  Data Type: Int8", indent),
          Ok(ArrayDataType::InvalidArrayDataType) | Err(_) => {}
        }
      }
      Type::DictionaryType(_) => println!("{}Type: Dictionary", indent),
      Type::SequenceType(_) => println!("{}Type: Sequence", indent),
      Type::StateType(_) => println!("{}Type: State", indent),
    }
  }

  if feature_type.is_optional {
    println!("{}Optional: true", indent);
  }
}

fn print_neural_network_details(layers: &[coreml_proto::proto::NeuralNetworkLayer]) {
  if !layers.is_empty() {
    println!("    Layers: {} total", layers.len());

    let mut layer_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for layer in layers {
      let layer_type_name = get_layer_type_name(layer);
      *layer_types.entry(layer_type_name).or_insert(0) += 1;
    }

    if !layer_types.is_empty() {
      println!("    Layer Types:");
      let mut types: Vec<_> = layer_types.iter().collect();
      types.sort_by_key(|&(name, _)| name);
      for (layer_type, count) in types {
        println!("      - {}: {}", layer_type, count);
      }
    }

    println!("    First 5 layers:");
    for (i, layer) in layers.iter().take(5).enumerate() {
      println!("      {}. Name: {}", i + 1, layer.name);
      println!("         Type: {}", get_layer_type_name(layer));
      if !layer.input.is_empty() {
        println!("         Inputs: {:?}", layer.input);
      }
      if !layer.output.is_empty() {
        println!("         Outputs: {:?}", layer.output);
      }
    }

    if layers.len() > 5 {
      println!("      ... and {} more layers", layers.len() - 5);
    }
  }
}

fn get_layer_type_name(layer: &coreml_proto::proto::NeuralNetworkLayer) -> String {
  use coreml_proto::proto::neural_network_layer::Layer;

  if let Some(ref layer_type) = layer.layer {
    match layer_type {
      Layer::Acos(_) => "Acos".to_string(),
      Layer::Acosh(_) => "Acosh".to_string(),
      Layer::Activation(_) => "Activation".to_string(),
      Layer::Add(_) => "Add".to_string(),
      Layer::AddBroadcastable(_) => "AddBroadcastable".to_string(),
      Layer::ArgMax(_) => "ArgMax".to_string(),
      Layer::ArgMin(_) => "ArgMin".to_string(),
      Layer::ArgSort(_) => "ArgSort".to_string(),
      Layer::Asin(_) => "Asin".to_string(),
      Layer::Asinh(_) => "Asinh".to_string(),
      Layer::Atan(_) => "Atan".to_string(),
      Layer::Atanh(_) => "Atanh".to_string(),
      Layer::Average(_) => "Average".to_string(),
      Layer::BatchedMatmul(_) => "BatchedMatmul".to_string(),
      Layer::Batchnorm(_) => "Batchnorm".to_string(),
      Layer::BiDirectionalLstm(_) => "BiDirectionalLSTM".to_string(),
      Layer::Bias(_) => "Bias".to_string(),
      Layer::Branch(_) => "Branch".to_string(),
      Layer::BroadcastToDynamic(_) => "BroadcastToDynamic".to_string(),
      Layer::BroadcastToLike(_) => "BroadcastToLike".to_string(),
      Layer::BroadcastToStatic(_) => "BroadcastToStatic".to_string(),
      Layer::CategoricalDistribution(_) => "CategoricalDistribution".to_string(),
      Layer::Ceil(_) => "Ceil".to_string(),
      Layer::ClampedReLu(_) => "ClampedReLU".to_string(),
      Layer::Clip(_) => "Clip".to_string(),
      Layer::Concat(_) => "Concat".to_string(),
      Layer::ConcatNd(_) => "ConcatND".to_string(),
      Layer::ConstantPad(_) => "ConstantPad".to_string(),
      Layer::Convolution(_) => "Convolution".to_string(),
      Layer::Convolution3d(_) => "Convolution3d".to_string(),
      Layer::Copy(_) => "Copy".to_string(),
      Layer::Cos(_) => "Cos".to_string(),
      Layer::Cosh(_) => "Cosh".to_string(),
      Layer::Crop(_) => "Crop".to_string(),
      Layer::CropResize(_) => "CropResize".to_string(),
      Layer::CumSum(_) => "Cumsum".to_string(),
      Layer::Custom(_) => "Custom".to_string(),
      Layer::DivideBroadcastable(_) => "DivideBroadcastable".to_string(),
      Layer::Dot(_) => "Dot".to_string(),
      Layer::Embedding(_) => "Embedding".to_string(),
      Layer::EmbeddingNd(_) => "EmbeddingND".to_string(),
      Layer::Equal(_) => "Equal".to_string(),
      Layer::Erf(_) => "Erf".to_string(),
      Layer::Exp2(_) => "Exp2".to_string(),
      Layer::ExpandDims(_) => "ExpandDims".to_string(),
      Layer::FillDynamic(_) => "FillDynamic".to_string(),
      Layer::FillLike(_) => "FillLike".to_string(),
      Layer::FillStatic(_) => "FillStatic".to_string(),
      Layer::Flatten(_) => "Flatten".to_string(),
      Layer::FlattenTo2D(_) => "FlattenTo2D".to_string(),
      Layer::Floor(_) => "Floor".to_string(),
      Layer::FloorDivBroadcastable(_) => "FloorDivBroadcastable".to_string(),
      Layer::Gather(_) => "Gather".to_string(),
      Layer::GatherAlongAxis(_) => "GatherAlongAxis".to_string(),
      Layer::GatherNd(_) => "GatherND".to_string(),
      Layer::Gelu(_) => "Gelu".to_string(),
      Layer::GetShape(_) => "GetShape".to_string(),
      Layer::GlobalPooling3d(_) => "GlobalPooling3d".to_string(),
      Layer::GreaterEqual(_) => "GreaterEqual".to_string(),
      Layer::GreaterThan(_) => "GreaterThan".to_string(),
      Layer::Gru(_) => "GRU".to_string(),
      Layer::InnerProduct(_) => "InnerProduct".to_string(),
      Layer::L2normalize(_) => "L2normalize".to_string(),
      Layer::LayerNormalization(_) => "LayerNormalization".to_string(),
      Layer::LessEqual(_) => "LessEqual".to_string(),
      Layer::LessThan(_) => "LessThan".to_string(),
      Layer::LoadConstant(_) => "LoadConstant".to_string(),
      Layer::LoadConstantNd(_) => "LoadConstantND".to_string(),
      Layer::LogicalAnd(_) => "LogicalAnd".to_string(),
      Layer::LogicalNot(_) => "LogicalNot".to_string(),
      Layer::LogicalOr(_) => "LogicalOr".to_string(),
      Layer::LogicalXor(_) => "LogicalXor".to_string(),
      Layer::Loop(_) => "Loop".to_string(),
      Layer::LoopBreak(_) => "LoopBreak".to_string(),
      Layer::LoopContinue(_) => "LoopContinue".to_string(),
      Layer::LowerTriangular(_) => "LowerTriangular".to_string(),
      Layer::Lrn(_) => "LRN".to_string(),
      Layer::MatrixBandPart(_) => "MatrixBandPart".to_string(),
      Layer::Max(_) => "Max".to_string(),
      Layer::MaxBroadcastable(_) => "MaxBroadcastable".to_string(),
      Layer::Min(_) => "Min".to_string(),
      Layer::MinBroadcastable(_) => "MinBroadcastable".to_string(),
      Layer::ModBroadcastable(_) => "ModBroadcastable".to_string(),
      Layer::Multiply(_) => "Multiply".to_string(),
      Layer::MultiplyBroadcastable(_) => "MultiplyBroadcastable".to_string(),
      Layer::Mvn(_) => "MVN".to_string(),
      Layer::NonMaximumSuppression(_) => "NonMaximumSuppression".to_string(),
      Layer::NotEqual(_) => "NotEqual".to_string(),
      Layer::OneHot(_) => "OneHot".to_string(),
      Layer::Padding(_) => "Padding".to_string(),
      Layer::Permute(_) => "Permute".to_string(),
      Layer::Pooling(_) => "Pooling".to_string(),
      Layer::Pooling3d(_) => "Pooling3d".to_string(),
      Layer::PowBroadcastable(_) => "PowBroadcastable".to_string(),
      Layer::RandomBernoulliDynamic(_) => "RandomBernoulliDynamic".to_string(),
      Layer::RandomBernoulliLike(_) => "RandomBernoulliLike".to_string(),
      Layer::RandomBernoulliStatic(_) => "RandomBernoulliStatic".to_string(),
      Layer::RandomNormalDynamic(_) => "RandomNormalDynamic".to_string(),
      Layer::RandomNormalLike(_) => "RandomNormalLike".to_string(),
      Layer::RandomNormalStatic(_) => "RandomNormalStatic".to_string(),
      Layer::RandomUniformDynamic(_) => "RandomUniformDynamic".to_string(),
      Layer::RandomUniformLike(_) => "RandomUniformLike".to_string(),
      Layer::RandomUniformStatic(_) => "RandomUniformStatic".to_string(),
      Layer::RangeDynamic(_) => "RangeDynamic".to_string(),
      Layer::RangeStatic(_) => "RangeStatic".to_string(),
      Layer::RankPreservingReshape(_) => "RankPreservingReshape".to_string(),
      Layer::Reduce(_) => "Reduce".to_string(),
      Layer::ReduceL1(_) => "ReduceL1".to_string(),
      Layer::ReduceL2(_) => "ReduceL2".to_string(),
      Layer::ReduceLogSum(_) => "ReduceLogSum".to_string(),
      Layer::ReduceLogSumExp(_) => "ReduceLogSumExp".to_string(),
      Layer::ReduceMax(_) => "ReduceMax".to_string(),
      Layer::ReduceMean(_) => "ReduceMean".to_string(),
      Layer::ReduceMin(_) => "ReduceMin".to_string(),
      Layer::ReduceProd(_) => "ReduceProd".to_string(),
      Layer::ReduceSum(_) => "ReduceSum".to_string(),
      Layer::ReduceSumSquare(_) => "ReduceSumSquare".to_string(),
      Layer::ReorganizeData(_) => "ReorganizeData".to_string(),
      Layer::Reshape(_) => "Reshape".to_string(),
      Layer::ReshapeDynamic(_) => "ReshapeDynamic".to_string(),
      Layer::ReshapeLike(_) => "ReshapeLike".to_string(),
      Layer::ReshapeStatic(_) => "ReshapeStatic".to_string(),
      Layer::ResizeBilinear(_) => "ResizeBilinear".to_string(),
      Layer::Reverse(_) => "Reverse".to_string(),
      Layer::ReverseSeq(_) => "ReverseSeq".to_string(),
      Layer::Round(_) => "Round".to_string(),
      Layer::Scale(_) => "Scale".to_string(),
      Layer::Scatter(_) => "Scatter".to_string(),
      Layer::ScatterAlongAxis(_) => "ScatterAlongAxis".to_string(),
      Layer::ScatterNd(_) => "ScatterND".to_string(),
      Layer::SequenceRepeat(_) => "SequenceRepeat".to_string(),
      Layer::Sign(_) => "Sign".to_string(),
      Layer::SimpleRecurrent(_) => "SimpleRecurrent".to_string(),
      Layer::Sin(_) => "Sin".to_string(),
      Layer::Sinh(_) => "Sinh".to_string(),
      Layer::Slice(_) => "Slice".to_string(),
      Layer::SliceBySize(_) => "SliceBySize".to_string(),
      Layer::SliceDynamic(_) => "SliceDynamic".to_string(),
      Layer::SliceStatic(_) => "SliceStatic".to_string(),
      Layer::SlidingWindows(_) => "SlidingWindows".to_string(),
      Layer::Softmax(_) => "Softmax".to_string(),
      Layer::SoftmaxNd(_) => "SoftmaxND".to_string(),
      Layer::Split(_) => "Split".to_string(),
      Layer::SplitNd(_) => "SplitND".to_string(),
      Layer::Squeeze(_) => "Squeeze".to_string(),
      Layer::Stack(_) => "Stack".to_string(),
      Layer::SubtractBroadcastable(_) => "SubtractBroadcastable".to_string(),
      Layer::Tan(_) => "Tan".to_string(),
      Layer::Tanh(_) => "Tanh".to_string(),
      Layer::Tile(_) => "Tile".to_string(),
      Layer::TopK(_) => "TopK".to_string(),
      Layer::Transpose(_) => "Transpose".to_string(),
      Layer::Unary(_) => "Unary".to_string(),
      Layer::UniDirectionalLstm(_) => "UniDirectionalLSTM".to_string(),
      Layer::UpperTriangular(_) => "UpperTriangular".to_string(),
      Layer::Upsample(_) => "Upsample".to_string(),
      Layer::WhereBroadcastable(_) => "WhereBroadcastable".to_string(),
      Layer::WhereNonZero(_) => "WhereNonZero".to_string(),
    }
  } else {
    "Unknown".to_string()
  }
}
