fn main() {
    // Define a default regressor from a dataset
    let mut model = automl::supervised::SupervisedModel::new_from_dataset(
        smartcore::dataset::diabetes::load_dataset(),
        automl::supervised::Settings::default_regression(),
    );

    // Run a model comparison with all models at default settings
    model.compare_models();
}
