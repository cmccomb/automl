fn main() {
    // Define a default regressor from a dataset
    let mut model = automl::supervised::SupervisedModel::new_from_dataset(
        smartcore::dataset::breast_cancer::load_dataset(),
        automl::supervised::Settings::default_classification(),
    );

    // Run a model comparison with all models at default settings
    model.compare_models();
}
