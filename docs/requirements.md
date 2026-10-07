# Timescape-Viewer Repository Requirements

- The application is implemented on top of the iced.rs GUI framework.
  - The application's dataflow and -storage must fit into iced's Elm architecture.
  - Long running processing must be executed in `iced::Task`s and utilize a central
    thread pool.
- A smooth UI has top priority. Computations or data handling tasks should never
  block the rendering in a way that we miss 60 frames per second.
- Virtually unbounded time series for long term trace.
  (Backed by on-disk SQLite database, when a threshold is crossed)
- Concurrent access. (Add samples while read by other threads)
- The application can show and manage several independent `Run`s or recordings.
- Runs might come from different sources such as one from a .csv file
  and one from a streaming source.
- Some file formats contain several runs in one file.
- Each run can contain any number of `Signal`s. If the names of two signals
  from two source are identical, consider the signals to be identical and
  treat the data as two instances from the same signal.
  For example the user might open both runs side by side and this one signal
  should be shown in each runs' window.
- Runs _can_ but _don't need_ to share signals. Sometimes two runs might contain
  totally different signals with zero overlap. But frequently all runs that a user
  looks at the same time have very similar sets of signals.
- The user can close a run any time and all associated resources must be freed.
  This imposes a challenge with shared signals.
- The data might be sampled at irregular intervals. In the first step we will
  focus on data with a constant sample rate as this variant is easy to cover
  with implicit in-order forest, but later the application has to support
  irregular sampling.
- Several signals might share a common sampling interval. Or in different words
  several signals might be sampled together. Those signals should share timestamp
  data to save memory and to keep the relationship.
- The application will support static files (finite number of samples) and live
  streaming sources. The user should be able to view all the data available until
  now and all new data will be added smoothly at 60 FPS on arrival.
- Small runs with maybe a few million data points will be kept entirely in RAM,
  larger data sets will be offloaded in a SQLite database to disk. Only the
  currently visible portions of the timeseries will be kept in memory.
- If the user views the entire timeseries with billions of datapoints at once,
  the data is sub-sampled to keep the RAM requirements in check. This is achieved
  through a variation of implicit in-order forest where - in contrast to the
  original application - the leafs might not be single samples but already
  aggregated ranges.
- The first draft will skip the SQLite offload and work only with the in-memory
  representation. The architecture however should be ready for the later addition
  of the SQLite offload to support giant time series.
